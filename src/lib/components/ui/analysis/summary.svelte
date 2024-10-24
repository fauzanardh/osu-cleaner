<script lang="ts">
	import { Trash2 } from 'lucide-svelte';

	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';

	import CategoryItem from '$lib/components/ui/analysis/category-item.svelte';
	import DeleteConfirmDialog from '$lib/components/ui/analysis/delete-confirm-dialog.svelte';
	import { humanizeFileSize } from '$lib/utils/humanize';
	import type {
		CategorySummaryResponse,
		CategoryUI,
		CategoryState
	} from '$lib/utils/interfaces.ts';
	import { getContext } from 'svelte';

	let {
		data = null,
		showDeleteConfirm = $bindable(false),
		isDeleting = $bindable(false),
		onCategoryExpandClick,
		onDelete,
		onDeleteComplete
	} = $props<{
		data: CategorySummaryResponse | null;
		showDeleteConfirm: boolean;
		isDeleting: boolean;
		onCategoryExpandClick: (category: string) => void;
		onDelete: (selectedCategories: string[]) => Promise<void>;
		onDeleteComplete: () => void;
	}>();

	let categoryContext = getContext<{
		categories: CategoryState;
		reset: () => void;
		updateCategory: (data: CategorySummaryResponse) => void;
		toggleCategorySelected: (category: string) => void;
	}>('category');

	const handleDelete = async () => {
		const selectedCategories = Object.values(categoryContext.categories)
			.filter((cat) => cat.selected)
			.map((cat) => cat.id);

		onDelete(selectedCategories);
	};

	let totalSize = $state<number>(0);
	$effect(() => {
		totalSize = Object.values(categoryContext.categories).reduce(
			(sum, cat) => sum + (cat.selected ? cat.size : 0),
			0
		);
	});

	let selectedCategories = $state<CategoryUI[]>([]);
	$effect(() => {
		selectedCategories = Object.values(categoryContext.categories).filter((cat) => cat.selected);
	});
</script>

{#if data}
	<Card.Root>
		<Card.Header>
			<Card.Title class="flex items-center gap-2">
				<Trash2 class="h-5 w-5" />
				Select Category to Clean
			</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="grid gap-2">
				{#each Object.values(categoryContext.categories) as category (category.id)}
					<CategoryItem
						{category}
						onSelect={(selected) => (categoryContext.categories[category.id].selected = selected)}
						{onCategoryExpandClick}
					/>
				{/each}
			</div>
		</Card.Content>
	</Card.Root>
	<div class="flex justify-end gap-4">
		<Button
			size="lg"
			variant="destructive"
			disabled={totalSize === 0}
			on:click={() => (showDeleteConfirm = true)}
		>
			<Trash2 class="h-5 w-5" />
			Delete Selected Files ({humanizeFileSize(totalSize)})
		</Button>
	</div>
	<DeleteConfirmDialog
		bind:show={showDeleteConfirm}
		{totalSize}
		{selectedCategories}
		onConfirm={handleDelete}
		onComplete={onDeleteComplete}
		bind:isDeleting
	/>
{/if}
