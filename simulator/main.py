#!/usr/bin/env python3
"""
Agent Simulator for AI Social Network Simulation

This script runs the main simulation engine that controls agent actions
based on their profiles, driving the simulation through API calls.
"""

import logging
import os

# Load environment variables from project root
import pathlib
import random
import sys
import time
from typing import Any

import requests
from dotenv import load_dotenv
from google import genai

project_root = pathlib.Path(__file__).parent.parent
load_dotenv(project_root / ".env")

# Configure logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


class AgentSimulator:
    def __init__(self):
        self.gemini_api_key = os.getenv("GEMINI_API_KEY")
        self.admin_api_key = os.getenv("ADMIN_API_KEY", "default_admin_key")
        self.api_base_url = os.getenv("API_BASE_URL", "http://localhost:3000")
        self.simulation_ticks = int(os.getenv("SIMULATION_TICKS", "100"))
        self.tick_duration = int(os.getenv("TICK_DURATION_SECONDS", "5"))

        if not self.gemini_api_key:
            raise ValueError("GEMINI_API_KEY environment variable is required")

        # Configure Gemini
        self.client = genai.Client(api_key=self.gemini_api_key)

        # API headers
        self.headers = {
            "Content-Type": "application/json",
            "X-Admin-API-Key": self.admin_api_key,
        }

        # Simulation state
        self.agents = []
        self.current_tick = 0

    def load_agents(self) -> bool:
        """Load all agent profiles from the API."""
        try:
            url = f"{self.api_base_url}/api/v1/agents"
            response = requests.get(url)

            if response.status_code == 200:
                self.agents = response.json()
                logger.info(f"Loaded {len(self.agents)} agents")
                return True
            else:
                logger.error(f"Failed to load agents: {response.status_code}")
                return False

        except requests.RequestException as e:
            logger.error(f"Network error loading agents: {e}")
            return False

    def should_agent_act(self, agent: dict[str, Any]) -> bool:
        """Determine if an agent should perform an action this tick."""
        frequency = agent.get("posting_frequency", "medium").lower()

        probability_map = {
            "very_high": 0.4,
            "high": 0.2,
            "medium": 0.1,
            "low": 0.05,
        }

        probability = probability_map.get(frequency, 0.1)
        return random.random() < probability

    def decide_action_type(self) -> str:
        """Randomly decide what type of action to perform."""
        actions = ["post", "like", "follow", "comment", "repost"]
        weights = [
            0.4,
            0.2,
            0.1,
            0.2,
            0.1,
        ]  # Posts still common, new interactions added
        return random.choices(actions, weights=weights)[0]

    def generate_post_content(self, agent: dict[str, Any]) -> str:
        """Generate post content for an agent using Gemini."""
        prompt = f"""
Generate a short social media post (max 280 characters) for this agent:

Name: {agent["name"]}
Persona: {agent["persona_summary"]}
Core Topics: {", ".join(agent["core_topics"])}
Content Style: {agent["content_style"]}
Behavioral Rules: {", ".join(agent["initial_behavioral_rules"])}

The post should:
- Match their personality and style
- Be about one of their core topics
- Follow their behavioral patterns
- Be realistic and engaging
- Stay under 280 characters

Return ONLY the post content, no quotes or extra text.
"""

        try:
            response = self.client.models.generate_content(
                model="gemini-2.5-flash-lite", contents=prompt
            )
            if (
                not response.candidates
                or not response.candidates[0].content
                or not response.candidates[0].content.parts
            ):
                return "Unable to generate content at this time."
            text = response.candidates[0].content.parts[0].text
            if not text:
                return "Unable to generate content at this time."
            content = text.strip()

            # Clean up the response
            content = content.strip('"').strip("'")
            if len(content) > 280:
                content = content[:277] + "..."

            return content

        except Exception as e:
            logger.error(f"Error generating content for {agent['name']}: {e}")
            # Fallback to a simple post based on their topics
            topic = random.choice(agent["core_topics"])
            return f"Thinking about {topic} today... #thoughts"

    def create_post(self, agent: dict[str, Any]) -> bool:
        """Create a post for an agent."""
        try:
            content = self.generate_post_content(agent)
            payload = {"agent_id": agent["id"], "content": content}
            url = f"{self.api_base_url}/api/v1/posts"

            response = requests.post(url, json=payload)

            if response.status_code == 201:
                logger.info(f"{agent['name']} posted: {content[:50]}...")
                return True
            else:
                logger.error(f"Failed to create post for {agent['name']}")
                return False

        except Exception as e:
            logger.error(f"Error creating post for {agent['name']}: {e}")
            return False

    def get_global_feed(self, limit: int = 20) -> list[dict[str, Any]]:
        """Get recent posts from the global feed for content discovery."""
        try:
            url = f"{self.api_base_url}/api/v1/posts/feed"
            response = requests.get(url)

            if response.status_code == 200:
                posts = response.json()[:limit]
                return posts
            else:
                logger.error("Failed to fetch global feed")
                return []
        except Exception as e:
            logger.error(f"Error fetching global feed: {e}")
            return []

    def get_recent_posts(self, limit: int = 10) -> list[dict[str, Any]]:
        """Get recent posts from the timeline (simplified - gets from any agent)."""
        try:
            # For simplicity, get timeline from the first agent
            if not self.agents:
                return []

            agent_id = self.agents[0]["id"]
            url = f"{self.api_base_url}/api/v1/agents/{agent_id}/timeline"
            response = requests.get(url)

            if response.status_code == 200:
                posts = response.json()
                return posts[:limit]
            else:
                return []

        except Exception as e:
            logger.error(f"Error getting recent posts: {e}")
            return []

    def like_random_post(self, agent: dict[str, Any]) -> bool:
        """Like a random post from the engagement-based global feed."""
        try:
            posts = self.get_global_feed(limit=15)
            if not posts:
                return False

            # Filter out agent's own posts
            other_posts = [p for p in posts if p["agent_id"] != agent["id"]]
            if not other_posts:
                return False

            post = random.choice(other_posts)
            payload = {"agent_id": agent["id"]}
            url = f"{self.api_base_url}/api/v1/posts/{post['id']}/like"

            response = requests.post(url, json=payload)

            if response.status_code == 201:
                logger.info(f"{agent['name']} liked a post")
                return True
            else:
                return False

        except Exception as e:
            logger.error(f"Error liking post for {agent['name']}: {e}")
            return False

    def follow_random_agent(self, agent: dict[str, Any]) -> bool:
        """Follow a random active agent discovered from the global feed."""
        try:
            # Get recent posts to discover active agents
            posts = self.get_global_feed(limit=20)
            if not posts:
                # Fallback to random agent selection
                other_agents = [a for a in self.agents if a["id"] != agent["id"]]
                if not other_agents:
                    return False
                target_id = random.choice(other_agents)["id"]
            else:
                # Find active agents from posts (those who are posting content)
                active_agent_ids = list(
                    {p["agent_id"] for p in posts if p["agent_id"] != agent["id"]}
                )
                if not active_agent_ids:
                    return False
                target_id = random.choice(active_agent_ids)

            payload = {"follower_id": agent["id"]}
            url = f"{self.api_base_url}/api/v1/agents/{target_id}/follow"

            response = requests.post(url, json=payload)

            if response.status_code == 201:
                # Find the target agent's name for logging
                target_name = f"Agent {target_id}"
                for a in self.agents:
                    if a["id"] == target_id:
                        target_name = a["name"]
                        break
                logger.info(f"{agent['name']} followed {target_name}")
                return True
            else:
                return False

        except Exception as e:
            logger.error(f"Error following for {agent['name']}: {e}")
            return False

    def comment_on_post(self, agent: dict[str, Any]) -> bool:
        """Make the agent comment on a random post from the global feed."""
        try:
            posts = self.get_global_feed(limit=10)
            if not posts:
                return False

            # Filter out agent's own posts
            other_posts = [p for p in posts if p["agent_id"] != agent["id"]]
            if not other_posts:
                return False

            post = random.choice(other_posts)

            # Generate a contextual comment using Gemini
            prompt = f"""
You are {agent["name"]}, commenting on this post: "{post["content"]}"

Your personality: {agent["persona_summary"]}
Your interests: {", ".join(agent["core_topics"][:3])}

Write a brief, authentic comment (1-2 sentences) that {agent["name"]} would make.
Keep it conversational and relevant to the original post.
"""

            try:
                response = self.client.models.generate_content(
                    model="gemini-2.5-flash-lite", contents=prompt
                )
                if (
                    not response.candidates
                    or not response.candidates[0].content
                    or not response.candidates[0].content.parts
                ):
                    content = "Interesting perspective!"
                else:
                    text = response.candidates[0].content.parts[0].text
                    if not text:
                        content = "Interesting perspective!"
                    else:
                        content = text.strip().strip('"').strip("'")
                        if len(content) > 280:
                            content = content[:277] + "..."
            except Exception:
                content = "Interesting perspective!"

            payload = {"agent_id": agent["id"], "content": content}
            url = f"{self.api_base_url}/api/v1/posts/{post['id']}/comments"

            response = requests.post(url, json=payload)

            if response.status_code == 201:
                logger.info(f"{agent['name']} commented: {content[:30]}...")
                return True
            else:
                return False

        except Exception as e:
            logger.error(f"Error commenting for {agent['name']}: {e}")
            return False

    def repost_content(self, agent: dict[str, Any]) -> bool:
        """Make the agent repost content from the global feed."""
        try:
            posts = self.get_global_feed(limit=10)
            if not posts:
                return False

            # Filter out agent's own posts
            other_posts = [p for p in posts if p["agent_id"] != agent["id"]]
            if not other_posts:
                return False

            post = random.choice(other_posts)

            # Optionally add a comment to the repost
            comment = None
            if random.random() < 0.5:  # 50% chance to add comment
                comment = f"This resonates with me! #{random.choice(agent['core_topics']).replace(' ', '')}"

            payload = {"agent_id": agent["id"]}
            if comment:
                payload["comment"] = comment

            url = f"{self.api_base_url}/api/v1/posts/{post['id']}/repost"

            response = requests.post(url, json=payload)

            if response.status_code == 201:
                action_text = (
                    f"reposted with comment: {comment}"
                    if comment
                    else "reposted content"
                )
                logger.info(f"{agent['name']} {action_text}")
                return True
            else:
                return False

        except Exception as e:
            logger.error(f"Error reposting for {agent['name']}: {e}")
            return False

    def perform_agent_action(self, agent: dict[str, Any]) -> bool:
        """Perform a random action for an agent."""
        action = self.decide_action_type()

        if action == "post":
            return self.create_post(agent)
        elif action == "like":
            return self.like_random_post(agent)
        elif action == "follow":
            return self.follow_random_agent(agent)
        elif action == "comment":
            return self.comment_on_post(agent)
        elif action == "repost":
            return self.repost_content(agent)

        return False

    def run_simulation_tick(self):
        """Run one tick of the simulation."""
        self.current_tick += 1
        logger.info(f"--- Simulation Tick {self.current_tick} ---")

        active_agents = 0
        successful_actions = 0

        for agent in self.agents:
            if self.should_agent_act(agent):
                active_agents += 1
                if self.perform_agent_action(agent):
                    successful_actions += 1

        logger.info(
            f"Tick {self.current_tick}: {active_agents} agents active, "
            f"{successful_actions} successful actions"
        )

    def reset_simulation(self) -> bool:
        """Reset the simulation state."""
        try:
            url = f"{self.api_base_url}/api/v1/admin/reset"
            response = requests.post(url, headers=self.headers)

            if response.status_code == 200:
                logger.info("Simulation reset successfully")
                return True
            else:
                logger.error(f"Failed to reset simulation: {response.status_code}")
                return False

        except requests.RequestException as e:
            logger.error(f"Network error resetting simulation: {e}")
            return False

    def get_simulation_stats(self):
        """Get final simulation statistics."""
        try:
            # Get all agents to see their final state
            url = f"{self.api_base_url}/api/v1/agents"
            response = requests.get(url)

            if response.status_code == 200:
                agents = response.json()
                logger.info("Final simulation stats:")
                logger.info(f"Total agents: {len(agents)}")
                # Note: Would need additional API endpoints to get post counts, etc.

        except Exception as e:
            logger.error(f"Error getting simulation stats: {e}")

    def run(self):
        """Main simulation execution."""
        logger.info("Starting AI Social Network Simulator")
        logger.info(
            f"Configuration: {self.simulation_ticks} ticks, "
            f"{self.tick_duration}s per tick"
        )

        # Load agents
        if not self.load_agents():
            logger.error("Failed to load agents. Exiting.")
            sys.exit(1)

        if not self.agents:
            logger.error("No agents available for simulation. Run initializer first.")
            sys.exit(1)

        # Main simulation loop
        try:
            for _ in range(self.simulation_ticks):
                self.run_simulation_tick()
                time.sleep(self.tick_duration)

        except KeyboardInterrupt:
            logger.info("Simulation interrupted by user")

        # Final statistics
        logger.info("Simulation completed!")
        self.get_simulation_stats()


def main():
    try:
        simulator = AgentSimulator()
        simulator.run()
    except KeyboardInterrupt:
        logger.info("Interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Fatal error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
