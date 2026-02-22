<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import { getCategoryIcon } from '$lib/utils/category';
	import { Download } from 'lucide-svelte';

	interface Props {
		profile: ProfileInfo | null;
		onInstall?: () => void;
	}

	let { profile, onInstall }: Props = $props();

	let IconComponent = $derived(profile ? getCategoryIcon(profile.category) : null);
</script>

{#if profile}
	<div class="hero-banner">
		<!-- Code text background -->
		<div class="code-bg" aria-hidden="true">
			<span>instanceof o&&A(f)&#123;&#125; instanceof</span>
			<span>.length&#125;for(&#123;</span>
			<span>push.i)&#123;if(cr=.length,icc==n.length&#125;for(</span>
			<span>&#123;&#123;attr:void 0&#125;&#125;.return t.pop()&#125;,e.pop</span>
			<span>return h(n)&#123;return h(n,r,t,e,u</span>
			<span>function v(n,r,t,e,u</span>
			<span>i=n.length,j=false,s&&e.length&#125;&#125;</span>
			<span>push.i)&#123;if(cr=.length</span>
		</div>

		<!-- Content -->
		<div class="hero-content">
			<!-- Left: icon + identity -->
			<div class="hero-identity">
				<div class="hero-icon">
					{#if IconComponent}
						<IconComponent class="w-7 h-7 text-white" />
					{/if}
				</div>
				<div class="hero-info">
					<div class="hero-title-row">
						<h1 class="hero-title">{profile.name}</h1>
						<span class="stable-badge">STABLE</span>
					</div>
					<p class="hero-meta">v{profile.version} • Curated by {profile.author}</p>
				</div>
			</div>

			<!-- Right: Install button -->
			<button class="install-btn" onclick={onInstall}>
				<Download class="w-4 h-4" />
				Install Pack
			</button>
		</div>
	</div>
{/if}

<style>
	/* ── Banner ───────────────────────────────────── */
	.hero-banner {
		position: relative;
		overflow: hidden;
		background: linear-gradient(135deg, #080d08 0%, #0c160c 50%, #0a110a 100%);
		padding: 1.75rem 2rem;
		flex-shrink: 0;
	}

	/* ── Code background text ─────────────────────── */
	.code-bg {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		flex-wrap: wrap;
		gap: 0.5rem;
		padding: 0.75rem 1.5rem;
		overflow: hidden;
		pointer-events: none;
		font-family: 'JetBrains Mono', 'Cascadia Code', 'Fira Code', ui-monospace, monospace;
		font-size: 0.75rem;
		line-height: 1.7;
		color: #4ade80;
		opacity: 0.1;
		white-space: nowrap;
		user-select: none;
	}

	/* ── Content ──────────────────────────────────── */
	.hero-content {
		position: relative;
		z-index: 1;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1.5rem;
	}

	/* ── Identity ─────────────────────────────────── */
	.hero-identity {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.hero-icon {
		width: 3.25rem;
		height: 3.25rem;
		border-radius: 0.875rem;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #22c55e, #16a34a);
		box-shadow: 0 4px 14px rgba(34, 197, 94, 0.35);
	}

	.hero-info {
		display: flex;
		flex-direction: column;
		gap: 0.2rem;
	}

	.hero-title-row {
		display: flex;
		align-items: center;
		gap: 0.6rem;
	}

	.hero-title {
		font-size: 1.35rem;
		font-weight: 800;
		color: #ffffff;
		margin: 0;
		letter-spacing: -0.015em;
		line-height: 1.2;
	}

	/* ── STABLE badge ─────────────────────────────── */
	.stable-badge {
		padding: 0.1rem 0.5rem;
		border-radius: 9999px;
		font-size: 0.58rem;
		font-weight: 800;
		text-transform: uppercase;
		letter-spacing: 0.07em;
		background: #22c55e;
		color: #052e16;
		line-height: 1.6;
	}

	.hero-meta {
		font-size: 0.775rem;
		color: rgba(255, 255, 255, 0.5);
		margin: 0;
	}

	/* ── Install button ───────────────────────────── */
	.install-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.6rem 1.35rem;
		border-radius: 0.65rem;
		border: none;
		background: #22c55e;
		color: #052e16;
		font-size: 0.82rem;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
		box-shadow: 0 3px 12px rgba(34, 197, 94, 0.3);
		flex-shrink: 0;
	}

	.install-btn:hover {
		background: #16a34a;
		transform: translateY(-1px);
		box-shadow: 0 6px 20px rgba(34, 197, 94, 0.4);
	}

	.install-btn:active {
		transform: translateY(0);
	}

	/* ── Responsive ───────────────────────────────── */
	@media (max-width: 600px) {
		.hero-banner {
			padding: 1.25rem 1.25rem;
		}

		.hero-content {
			flex-direction: column;
			align-items: flex-start;
		}

		.install-btn {
			width: 100%;
			justify-content: center;
		}
	}
</style>
