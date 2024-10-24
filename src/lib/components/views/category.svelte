<script lang="ts">
	import { ArrowLeft, FileQuestion } from 'lucide-svelte';
	import type { ComponentType } from 'svelte';
	import VirtualList from 'svelte-tiny-virtual-list';

	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card';
	import * as Select from '$lib/components/ui/select/index.js';

	import { FileProcessorService } from '$lib/services/file_processor';
	import { humanizeFileSize } from '$lib/utils/humanize';
	import type { CategoryUI, FileInfo } from '$lib/utils/interfaces';

	let { selectedCategory, setCurrentView } = $props<{
		selectedCategory: CategoryUI | null;
		setCurrentView: (view: string) => void;
	}>();

	const fileProcessor = new FileProcessorService();

	let selectedCategoryFiles = $state<FileInfo[]>([]);
	let SelectedCategoryIcon = $state<ComponentType>(FileQuestion); // Default icon
	$effect(() => {
		if (selectedCategory) {
			SelectedCategoryIcon = selectedCategory.icon;
			fileProcessor.getCategoryData(selectedCategory.id).then((data) => {
				selectedCategoryFiles = data.files;
			});
		}
	});

	const sortCategoryFiles = (option: string) => {
		if (option === 'name-asc') {
			selectedCategoryFiles = selectedCategoryFiles.sort((a, b) =>
				getFileName(a.path).localeCompare(getFileName(b.path))
			);
		} else if (option === 'name-desc') {
			selectedCategoryFiles = selectedCategoryFiles.sort((a, b) =>
				getFileName(b.path).localeCompare(getFileName(a.path))
			);
		} else if (option === 'size-asc') {
			selectedCategoryFiles = selectedCategoryFiles.sort((a, b) => a.size - b.size);
		} else if (option === 'size-desc') {
			selectedCategoryFiles = selectedCategoryFiles.sort((a, b) => b.size - a.size);
		} else {
			selectedCategoryFiles = selectedCategoryFiles.sort((a, b) => a.path.localeCompare(b.path));
		}
	};

	const getFileName = (path: string) => {
		const parts = path.replace(/\\/g, '/').split('/');
		return parts[parts.length - 1];
	};
</script>

<div class="space-y-4">
	<div class="flex items-center justify-between">
		<Button variant="ghost" on:click={() => setCurrentView('main')} class="flex items-center gap-2">
			<ArrowLeft class="h-5 w-5" />
			Back to Summary
		</Button>

		<div class="relative">
			<Select.Root onSelectedChange={(v) => sortCategoryFiles(v?.value as string)}>
				<Select.Trigger class="w-[180px]">
					<Select.Value placeholder="Default" />
				</Select.Trigger>
				<Select.Content>
					<Select.Item value="default">Default</Select.Item>
					<Select.Item value="name-asc">Name (A-Z)</Select.Item>
					<Select.Item value="name-desc">Name (Z-A)</Select.Item>
					<Select.Item value="size-asc">Size (Smallest)</Select.Item>
					<Select.Item value="size-desc">Size (Largest)</Select.Item>
				</Select.Content>
			</Select.Root>
		</div>
	</div>

	{#if selectedCategory}
		<!-- File list -->
		<Card.Root>
			<Card.Header>
				<Card.Title class="flex items-center justify-between">
					<div class="flex items-center gap-2">
						<SelectedCategoryIcon class="h-5 w-5" />
						{selectedCategory.title}
					</div>
					<span class="text-sm text-gray-400">{selectedCategory.count} files</span>
				</Card.Title>
			</Card.Header>
			<Card.Content>
				{#if selectedCategoryFiles.length > 0}
					<VirtualList width="100%" height={480} itemCount={selectedCategory.count} itemSize={100}>
						<div
							slot="item"
							let:index
							let:style
							class="flex items-center justify-between space-x-4 border-zinc-700 px-4"
							{style}
						>
							<div class="flex-1">
								<p class="text-medium">{getFileName(selectedCategoryFiles[index].path)}</p>
								<p class="text-xs text-gray-400">{selectedCategoryFiles[index].path}</p>
							</div>
							<span class="text-xs text-gray-400"
								>{humanizeFileSize(selectedCategoryFiles[index].size)}</span
							>
						</div>
					</VirtualList>
				{:else}
					<div class="flex h-[480px] items-center justify-center">
						<p class="text-gray-400">No files available in this category.</p>
					</div>
				{/if}
			</Card.Content>
		</Card.Root>
	{/if}
</div>
