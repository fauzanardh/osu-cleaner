<script lang="ts">
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { ChevronRight } from 'lucide-svelte';

	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';

	import { humanizeFileSize } from '$lib/utils/humanize';
	import type { CategoryUI } from '$lib/utils/interfaces.ts';

	let { category, onSelect, onCategoryExpandClick } = $props<{
		category: CategoryUI;
		onSelect: (selected: boolean) => void;
		onCategoryExpandClick: (category: string) => void;
	}>();

	let isChecked = $state<boolean>(category.selected);
	$effect(() => {
		if (isChecked !== category.selected) {
			onSelect(isChecked);
		}
	});

	let showDialog = $state<boolean>(false);
	const beforeCategoryExpandClick = () => {
		if (category.count > 100000) {
			showDialog = true;
		} else {
			onCategoryExpandClick(category.id);
		}
	};
</script>

<AlertDialog.Root bind:open={showDialog}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>You are about to open a large category</AlertDialog.Title>
			<AlertDialog.Description>
				This category contains many files and might take several seconds to load. Would you like to
				proceed?
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action asChild>
				<Button
					on:click={() => {
						showDialog = false;
						onCategoryExpandClick(category.id);
					}}>Continue</Button
				>
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>

<div class="flex items-center gap-4 rounded-lg border border-zinc-800 px-4 py-2">
	<div class="flex flex-1 items-center gap-3">
		<Checkbox
			id={`${category.id}-checkbox`}
			bind:checked={isChecked}
			aria-label={`Select ${category.title}`}
		/>
		<category.icon class="h-5 w-5"></category.icon>
		<div>
			<h3 class="font-medium">{category.title}</h3>
			<p class="text-sm text-gray-400">{category.description}</p>
		</div>
	</div>
	<div class="flex flex-col items-end text-right">
		<span class="text-sm font-medium">{humanizeFileSize(category.size)}</span>
		<span class="text-xs text-gray-400">{Math.round(category.count)} files</span>
	</div>
	<button onclick={() => beforeCategoryExpandClick()}>
		<ChevronRight class="h-5 w-5" />
	</button>
</div>
