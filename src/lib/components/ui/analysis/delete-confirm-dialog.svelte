<script lang="ts">
	import { cn } from '$lib/utils';
	import { listen, type Event, type UnlistenFn } from '@tauri-apps/api/event';
	import { AlertCircle, CheckCircle2, Loader2, XCircle } from 'lucide-svelte';
	import { getContext } from 'svelte';
	import { quintOut } from 'svelte/easing';
	import { fly } from 'svelte/transition';

	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';

	import { deletion } from '$lib/consts';
	import { humanizeFileSize } from '$lib/utils/humanize';
	import type { AlertMessage, CategoryUI } from '$lib/utils/interfaces';
	import { FileProcessorService } from '$lib/services/file_processor';

	let {
		show = $bindable(false),
		totalSize,
		selectedCategories = [],
		onConfirm,
		onComplete,
		isDeleting = $bindable(false)
	} = $props<{
		show: boolean;
		totalSize: number;
		selectedCategories: CategoryUI[];
		onConfirm: () => Promise<void>;
		onComplete: () => void;
		isDeleting: boolean;
	}>();

	const alertContext = getContext<{
		alerts: AlertMessage[];
		dismiss: (id: string) => void;
		show: (alert: Omit<AlertMessage, 'id'>) => void;
		clear: () => void;
	}>('alerts');

	const fileProcessor = new FileProcessorService();

	let error: string | null = $state<string | null>(null);
	let currentCategory = $state<CategoryUI | null>(null);
	let completedCategories = $state<Set<string>>(new Set());
	let isSuccess = $state<boolean>(false);
	let unsubscriber: Promise<UnlistenFn>[] = [];

	const categoryDeletionStartHandler = (event: Event<string>) => {
		currentCategory = selectedCategories.find(
			(category: CategoryUI) => category.id === event.payload
		);
		isDeleting = true;
	};

	const categoryDeletionCompleteHandler = (event: Event<string>) => {
		completedCategories.add(event.payload);
		if (completedCategories.size === selectedCategories.length) {
			isSuccess = true;
			isDeleting = false;
			currentCategory = null;
		}
	};

	$effect(() => {
		unsubscriber = [
			listen<string>(deletion.CATEGORY_START, categoryDeletionStartHandler),
			listen<string>(deletion.CATEGORY_COMPLETE, categoryDeletionCompleteHandler)
		];

		return () => {
			unsubscriber.forEach((unsub) => unsub.then((fn) => fn()));
		};
	});

	const handleDelete = async () => {
		if (isDeleting) return;

		error = null;
		isSuccess = false;
		completedCategories.clear();
		currentCategory = null;

		try {
			await onConfirm();
		} catch (err) {
			error = err instanceof Error ? err.message : 'Failed to delete files';
			isDeleting = false;
		}
	};

	const cancelDelete = async () => {
		if (isDeleting) {
			await fileProcessor.cancelOperation();
			alertContext.show({
				type: 'warning',
				title: 'Deletion Cancelled',
				message: 'Deletion operation was cancelled'
			});
		}
		show = false;
	};

	$effect(() => {
		if (show) {
			error = null;
			isDeleting = false;
			isSuccess = false;
			currentCategory = null;
			completedCategories.clear();
		}
	});

	const getCategoryStatus = (category: string) => {
		if (completedCategories.has(category)) {
			return { icon: CheckCircle2, class: 'text-green-500' };
		}
		if (currentCategory && currentCategory.id === category) {
			return { icon: Loader2, class: 'text-blue-500 animate-spin' };
		}

		return null;
	};
</script>

<AlertDialog.Root bind:open={show}>
	<AlertDialog.Content class="rounded-lg">
		{#if isSuccess}
			<div in:fly={{ y: 20, duration: 300, easing: quintOut }} out:fly={{ y: -20, duration: 200 }}>
				<AlertDialog.Header>
					<AlertDialog.Title class="flex items-center gap-2 text-green-500">
						<CheckCircle2 class="h-5 w-5" />
						Deletion Complete
					</AlertDialog.Title>
					<AlertDialog.Description class="space-y-4">
						<p>
							Successfully deleted {humanizeFileSize(totalSize)} of files across {selectedCategories.length}
							categories.
						</p>
						<!-- Spacer -->
						<div></div>
					</AlertDialog.Description>
				</AlertDialog.Header>
				<AlertDialog.Footer>
					<AlertDialog.Action asChild>
						<Button
							on:click={() => {
								show = false;
								onComplete();
							}}>Close</Button
						>
					</AlertDialog.Action>
				</AlertDialog.Footer>
			</div>
		{:else}
			<div in:fly={{ y: 20, duration: 300, easing: quintOut }} out:fly={{ y: -20, duration: 200 }}>
				<AlertDialog.Header>
					<AlertDialog.Title class="flex items-center gap-2">
						<AlertCircle class="h-5 w-5 text-red-500" />
						Confirm Deletion
					</AlertDialog.Title>
					<AlertDialog.Description class="space-y-4">
						<p>
							You're about to delete {humanizeFileSize(totalSize)} of files. This action cannot be undone.
						</p>

						{#if selectedCategories.length > 0}
							<div class="rounded-md bg-zinc-900 p-3 text-sm">
								<p class="mb-2 font-medium text-zinc-400">Selected categories:</p>
								<ul class="space-y-1 text-zinc-500">
									{#each selectedCategories as category}
										{@const status = getCategoryStatus(category.id)}
										<li
											class="relative flex items-center gap-2 pl-4 before:absolute before:left-0 before:text-zinc-500 before:content-['•']"
										>
											<span>{category.title}</span>
											{#if status}
												<status.icon class={cn('h-4 w-4', status.class)} />
											{:else}
												<div class="h-4 w-4"></div>
											{/if}
										</li>
									{/each}
								</ul>
							</div>
						{/if}

						{#if error}
							<div
								class="rounded-md bg-red-500/15 p-3 text-sm text-red-500"
								in:fly={{ y: 20, duration: 300, easing: quintOut }}
							>
								<div class="flex items-center gap-2">
									<XCircle class="h-5 w-5" />
									{error}
								</div>
							</div>
						{/if}
						<!-- Spacer -->
						<div></div>
					</AlertDialog.Description>
				</AlertDialog.Header>
				<AlertDialog.Footer class="gap-2">
					<AlertDialog.Cancel asChild>
						<Button on:click={cancelDelete}>Cancel</Button>
					</AlertDialog.Cancel>
					<AlertDialog.Action asChild>
						<Button variant="destructive" disabled={isDeleting} on:click={handleDelete}>
							{#if isDeleting}
								<Loader2 class="mr-2 h-4 w-4 animate-spin" />
								{#if currentCategory}
									Deleting {currentCategory.title}...
								{:else}
									Deleting...
								{/if}
							{:else}
								Delete Files
							{/if}
						</Button>
					</AlertDialog.Action>
				</AlertDialog.Footer>
			</div>
		{/if}
	</AlertDialog.Content>
</AlertDialog.Root>
