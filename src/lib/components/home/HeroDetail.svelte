<script lang="ts">
	import type { ProfileInfo } from '$lib/commands';
	import { getCategoryIcon } from '$lib/utils/category';
	import { Download, Trash2 } from '@lucide/svelte';

	interface Props {
		profile: ProfileInfo | null;
		allInstalled?: boolean;
		onInstall?: () => void;
		onUninstall?: () => void;
	}

	let { profile, allInstalled = false, onInstall, onUninstall }: Props = $props();

	let IconComponent = $derived(profile ? getCategoryIcon(profile.category) : null);
</script>

{#if profile}
	<div class="hero-banner" class:hero-banner--danger={allInstalled}>
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
				<div class="hero-icon" class:hero-icon--danger={allInstalled}>
					{#if IconComponent}
						<IconComponent class="w-7 h-7 text-white" />
					{/if}
				</div>
				<div class="hero-info">
					<div class="hero-title-row">
						<h1 class="hero-title">{profile.name}</h1>
						<span class="stable-badge" class:stable-badge--danger={allInstalled}>
							{allInstalled ? 'INSTALLED' : 'STABLE'}
						</span>
					</div>
					<p class="hero-meta">v{profile.version} • Curated by {profile.author}</p>
				</div>
			</div>

			<!-- Right: Install / Uninstall button -->
			{#if allInstalled}
				<button class="uninstall-btn" onclick={onUninstall}>
					<Trash2 class="w-4 h-4" />
					Uninstall Pack
				</button>
			{:else}
				<button class="install-btn" onclick={onInstall}>
					<Download class="w-4 h-4" />
					Install Pack
				</button>
			{/if}
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
		transition: background 0.3s ease;
	}

	.hero-banner--danger {
		background: linear-gradient(135deg, #0d0808 0%, #160c0c 50%, #110a0a 100%);
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

	.hero-banner--danger .code-bg {
		color: #f87171;
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
		transition: all 0.3s ease;
	}

	.hero-icon--danger {
		background: linear-gradient(135deg, #ef4444, #dc2626);
		box-shadow: 0 4px 14px rgba(239, 68, 68, 0.35);
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
		transition: all 0.3s ease;
	}

	.stable-badge--danger {
		background: #ef4444;
		color: #fff;
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

	/* ── Uninstall button ─────────────────────────── */
	.uninstall-btn {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.6rem 1.35rem;
		border-radius: 0.65rem;
		border: none;
		background: #ef4444;
		color: #ffffff;
		font-size: 0.82rem;
		font-weight: 700;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
		box-shadow: 0 3px 12px rgba(239, 68, 68, 0.3);
		flex-shrink: 0;
	}

	.uninstall-btn:hover {
		background: #dc2626;
		transform: translateY(-1px);
		box-shadow: 0 6px 20px rgba(239, 68, 68, 0.4);
	}

	.uninstall-btn:active {
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

		.install-btn,
		.uninstall-btn {
			width: 100%;
			justify-content: center;
		}
	}
</style>
