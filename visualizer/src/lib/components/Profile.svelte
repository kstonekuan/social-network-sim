<script lang="ts">
  import { onMount } from "svelte";
  import { ApiService } from "../api";
  import type { Agent, Post as PostType } from "../types";
  import Post from "./Post.svelte";

  interface Props {
    agent: Agent;
  }

  let { agent }: Props = $props();

  let posts = $state<PostType[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  onMount(async () => {
    await loadAgentPosts();
  });

  async function loadAgentPosts() {
    try {
      loading = true;
      error = null;
      posts = await ApiService.getAgentTimeline(agent.id);
    } catch (err) {
      error = `Failed to load posts for ${agent.name}`;
      console.error("Error loading agent posts:", err);
    } finally {
      loading = false;
    }
  }

  // Effect to reload posts when agent changes
  $effect(() => {
    if (agent) {
      loadAgentPosts();
    }
  });
</script>

<div class="max-w-2xl mx-auto">
  <!-- Profile Header -->
  <div class="profile-section">
    <div class="flex items-start space-x-4">
      <!-- Avatar -->
      <div
        class="w-20 h-20 bg-blue-500 rounded-full flex items-center justify-center text-white text-2xl font-bold"
      >
        {agent.name.charAt(0).toUpperCase()}
      </div>

      <div class="flex-1">
        <h1 class="text-2xl font-bold text-slate-900 dark:text-slate-100 mb-1">
          {agent.name}
        </h1>
        <p class="text-slate-600 dark:text-slate-400 mb-2">@agent{agent.id}</p>

        <!-- Bio/Persona Summary -->
        <p class="text-slate-800 dark:text-slate-300 mb-4">
          {agent.persona_summary}
        </p>

        <!-- Agent Details -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
          <div>
            <h3 class="font-semibold text-slate-700 dark:text-slate-300 mb-2">
              Core Topics
            </h3>
            <div class="flex flex-wrap gap-1">
              {#each agent.core_topics as topic}
                <span
                  class="bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200 px-2 py-1 rounded-full text-xs"
                >
                  {topic}
                </span>
              {/each}
            </div>
          </div>

          <div>
            <h3 class="font-semibold text-slate-700 dark:text-slate-300 mb-2">
              Details
            </h3>
            <p class="text-slate-600 dark:text-slate-400">
              <span class="font-medium">Posting Frequency:</span>
              {agent.posting_frequency}
            </p>
            <p class="text-slate-600 dark:text-slate-400">
              <span class="font-medium">Content Style:</span>
              {agent.content_style}
            </p>
          </div>
        </div>

        {#if agent.initial_behavioral_rules.length > 0}
          <div class="mt-4">
            <h3 class="font-semibold text-slate-700 dark:text-slate-300 mb-2">
              Behavioral Rules
            </h3>
            <ul class="text-sm text-slate-600 dark:text-slate-400 space-y-1">
              {#each agent.initial_behavioral_rules as rule}
                <li class="flex items-start">
                  <span class="text-slate-400 dark:text-slate-500 mr-2">•</span>
                  {rule}
                </li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Posts Section -->
  <div class="mb-6">
    <h2 class="text-xl font-bold text-slate-900 dark:text-slate-100 mb-4">
      Posts by {agent.name}
    </h2>

    {#if loading}
      <div class="flex justify-center items-center py-12">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500 dark:border-blue-400"
        ></div>
      </div>
    {:else if error}
      <div
        class="bg-red-100 border border-red-400 text-red-700 dark:bg-red-900 dark:border-red-700 dark:text-red-300 px-4 py-3 rounded relative"
        role="alert"
      >
        {error}
        <button class="ml-2 underline" onclick={loadAgentPosts}
          >Try again</button
        >
      </div>
    {:else if posts.length === 0}
      <div class="text-center py-12 text-slate-500 dark:text-slate-400">
        {agent.name} hasn't posted anything yet.
      </div>
    {:else}
      <div class="space-y-4">
        {#each posts as post (post.id)}
          <Post {post} />
        {/each}
      </div>
    {/if}
  </div>
</div>
