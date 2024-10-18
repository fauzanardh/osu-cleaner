<script lang="ts">
	import { Loader2 } from 'lucide-svelte';

	import StatCard from '$lib/components/ui/analysis/stat-card.svelte';
	import * as Card from '$lib/components/ui/card';
	import type { FilterCounts } from '$lib/utils/interfaces';

	let {
		title = '',
		subtitle = '',
		counts
	} = $props<{
		title: string;
		subtitle: string;
		counts: FilterCounts;
	}>();

	const statCards = $derived([
		{ id: 'background_video', count: counts.background_videos, label: 'Background Videos Found' },
		{ id: 'background_image', count: counts.background_images, label: 'Background Images Found' },
		{ id: 'storyboard', count: counts.storyboards, label: 'Storyboards Found' },
		{ id: 'hitsound', count: counts.hitsounds, label: 'Hitsounds Found' },
		{ id: 'skin_element', count: counts.skin_elements, label: 'Skin Elements Found' },
		{ id: 'other', count: counts.other, label: 'Other Found' }
	]);
</script>

<Card.Root>
	<Card.Header>
		<Card.Title class="flex items-center gap-2">
			<Loader2 class="h-5 w-5 animate-spin" aria-hidden="true" />
			<div class="flex flex-col p-4">
				<h2>{title}</h2>
				<span class="text-xs text-gray-400">{subtitle}</span>
			</div>
		</Card.Title>
	</Card.Header>
	<Card.Content>
		<div class="grid grid-cols-2 gap-2" role="region" aria-label="File analysis statistics">
			{#each statCards as { id, count, label }}
				<StatCard {id} {count} {label} />
			{/each}
		</div>
	</Card.Content>
</Card.Root>
