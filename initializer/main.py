#!/usr/bin/env python3
"""
Initializer Service for AI Social Network Simulation

This script generates AI agent profiles using the Gemini API and populates
them into the Twitter API Service database.
"""

import json
import logging
import os

# Load environment variables from project root
import pathlib
import sys
import time
from typing import Any

import requests
from dotenv import load_dotenv
from google import genai
from tqdm import tqdm

project_root = pathlib.Path(__file__).parent.parent
load_dotenv(project_root / ".env")

# Configure logging
logging.basicConfig(
    level=logging.INFO, format="%(asctime)s - %(levelname)s - %(message)s"
)
logger = logging.getLogger(__name__)


class InitializerService:
    def __init__(self):
        self.gemini_api_key = os.getenv("GEMINI_API_KEY")
        self.admin_api_key = os.getenv("ADMIN_API_KEY", "default_admin_key")
        self.api_base_url = os.getenv("API_BASE_URL", "http://localhost:3000")

        if not self.gemini_api_key:
            raise ValueError("GEMINI_API_KEY environment variable is required")

        # Configure Gemini
        self.client = genai.Client(api_key=self.gemini_api_key)

        # API headers
        self.headers = {
            "Content-Type": "application/json",
            "X-Admin-API-Key": self.admin_api_key,
        }

    def load_influencers(self) -> list[str]:
        """Load the list of influencers from JSON file."""
        try:
            with open("influencers.json") as f:
                influencers = json.load(f)
            logger.info(f"Loaded {len(influencers)} influencers")
            return influencers
        except FileNotFoundError:
            logger.error("influencers.json file not found")
            return []
        except json.JSONDecodeError as e:
            logger.error(f"Error parsing influencers.json: {e}")
            return []

    def generate_agent_profile(self, influencer_name: str) -> dict[str, Any]:
        """Generate an AI agent profile using Gemini API."""
        prompt = f"""
Create a detailed AI agent profile for a social media simulation based on the real person: {influencer_name}

You must return ONLY a valid JSON object with the following exact structure:
{{
  "name": "string - The person's name",
  "persona_summary": "string - A comprehensive 2-3 sentence summary of their personality, background, and public persona",
  "core_topics": ["array", "of", "strings", "representing", "their", "main", "topics", "of", "interest"],
  "posting_frequency": "string - One of: 'very_high', 'high', 'medium', 'low'",
  "content_style": "string - Detailed description of their writing style, tone, and typical content approach",
  "initial_behavioral_rules": ["array", "of", "strings", "describing", "specific", "behavioral", "patterns", "and", "interaction", "preferences"]
}}

Guidelines:
- Make the profile realistic and based on the public persona of {influencer_name}
- Include 5-8 core topics that reflect their interests/expertise
- Content style should be detailed (2-3 sentences) describing tone, humor, formality, etc.
- Include 4-6 behavioral rules that capture how they typically interact online
- Keep all strings concise but informative
- Ensure the JSON is valid and properly formatted

Return ONLY the JSON object, no other text.
"""

        try:
            # Removed redundant logging since tqdm will show progress
            response = self.client.models.generate_content(
                model="gemini-2.5-flash", contents=prompt
            )

            # Parse the JSON response
            if (
                not response.candidates
                or not response.candidates[0].content
                or not response.candidates[0].content.parts
            ):
                raise ValueError("Invalid response structure from Gemini API")
            profile_text = response.candidates[0].content.parts[0].text
            if not profile_text:
                raise ValueError("Empty response from Gemini API")
            profile_text = profile_text.strip()

            # Clean up common formatting issues
            if profile_text.startswith("```json"):
                profile_text = profile_text[7:]
            if profile_text.endswith("```"):
                profile_text = profile_text[:-3]

            profile = json.loads(profile_text)

            # Validate required fields
            required_fields = [
                "name",
                "persona_summary",
                "core_topics",
                "posting_frequency",
                "content_style",
                "initial_behavioral_rules",
            ]
            for field in required_fields:
                if field not in profile:
                    raise ValueError(f"Missing required field: {field}")

            logger.info(f"Successfully generated profile for {influencer_name}")
            return profile

        except json.JSONDecodeError as e:
            logger.error(f"Failed to parse JSON response for {influencer_name}: {e}")
            return {}
        except Exception as e:
            logger.error(f"Error generating profile for {influencer_name}: {e}")
            return {}

    def create_agent_via_api(self, profile: dict[str, Any]) -> bool:
        """Send agent profile to the Twitter API Service."""
        try:
            url = f"{self.api_base_url}/api/v1/admin/agents"
            response = requests.post(url, json=profile, headers=self.headers)

            if response.status_code == 201:
                logger.info(f"Successfully created agent: {profile['name']}")
                return True
            else:
                logger.error(
                    f"Failed to create agent {profile['name']}: {response.status_code} - {response.text}"
                )
                return False

        except requests.RequestException as e:
            logger.error(f"Network error creating agent {profile['name']}: {e}")
            return False

    def reset_simulation(self) -> bool:
        """Reset the simulation by clearing dynamic data."""
        try:
            url = f"{self.api_base_url}/api/v1/admin/reset"
            response = requests.post(url, headers=self.headers)

            if response.status_code == 200:
                logger.info("Successfully reset simulation data")
                return True
            else:
                logger.error(
                    f"Failed to reset simulation: {response.status_code} - {response.text}"
                )
                return False

        except requests.RequestException as e:
            logger.error(f"Network error resetting simulation: {e}")
            return False

    def run(self):
        """Main execution method."""
        logger.info("Starting AI Social Network Initializer")

        # Load influencers
        influencers = self.load_influencers()
        if not influencers:
            logger.error("No influencers loaded. Exiting.")
            sys.exit(1)

        # Reset simulation first
        logger.info("Resetting simulation...")
        if not self.reset_simulation():
            logger.warning("Failed to reset simulation, continuing anyway...")

        # Generate and create agents
        successful_creations = 0
        failed_creations = 0

        with tqdm(total=len(influencers), desc="Creating agents", unit="agent") as pbar:
            for influencer in influencers:
                try:
                    # Update progress bar description
                    pbar.set_description(f"Creating {influencer}")
                    
                    # Generate profile
                    profile = self.generate_agent_profile(influencer)
                    if not profile:
                        failed_creations += 1
                        pbar.set_postfix({"Success": successful_creations, "Failed": failed_creations})
                        pbar.update(1)
                        continue

                    # Create agent via API
                    if self.create_agent_via_api(profile):
                        successful_creations += 1
                    else:
                        failed_creations += 1
                    
                    # Update progress bar with current stats
                    pbar.set_postfix({"Success": successful_creations, "Failed": failed_creations})
                    pbar.update(1)

                    # Rate limiting - wait between requests
                    time.sleep(2)

                except Exception as e:
                    logger.error(f"Unexpected error processing {influencer}: {e}")
                    failed_creations += 1
                    pbar.set_postfix({"Success": successful_creations, "Failed": failed_creations})
                    pbar.update(1)

        # Summary
        logger.info("Initialization complete!")
        logger.info(f"Successful agent creations: {successful_creations}")
        logger.info(f"Failed agent creations: {failed_creations}")

        if failed_creations > 0:
            logger.warning(f"{failed_creations} agents failed to be created")
            sys.exit(1)


def main():
    try:
        initializer = InitializerService()
        initializer.run()
    except KeyboardInterrupt:
        logger.info("Interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Fatal error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
