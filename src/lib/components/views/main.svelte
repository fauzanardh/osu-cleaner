<script lang="ts">
	import { listen, type Event, type UnlistenFn } from '@tauri-apps/api/event';
	import { open } from '@tauri-apps/plugin-dialog';
	import { FolderOpen, Search } from 'lucide-svelte';
	import { getContext } from 'svelte';

	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input/index.js';

	import Analyzer from '$lib/components/ui/analysis/analyzer.svelte';
	import Summary from '$lib/components/ui/analysis/summary.svelte';

	import { file_processor, statusValues } from '$lib/consts';
	import { FileProcessorService } from '$lib/services/file_processor';
	import type {
		AlertMessage,
		AnalyzerState,
		CategoryState,
		CategorySummaryResponse,
		CounterUpdate
	} from '$lib/utils/interfaces';

	let { selectedDirectory = $bindable(''), onCategoryExpandClick } = $props<{
		selectedDirectory: string;
		onCategoryExpandClick: (category: string) => void;
	}>();

	const analyzerContext = getContext<{
		analyzer: AnalyzerState;
		reset: () => void;
		setStatus: (status: AnalyzerState['status']) => void;
		updateScanCount: (count: number) => void;
		updateParseCount: (count: number) => void;
		updateFilterCounts: (counts: CounterUpdate) => void;
		updateUI: (title: string, subtitle: string) => void;
		setSummary: (summary: AnalyzerState['summary']) => void;
	}>('analyzer');

	const alertContext = getContext<{
		alerts: AlertMessage[];
		dismiss: (id: string) => void;
		show: (alert: Omit<AlertMessage, 'id'>) => void;
		clear: () => void;
	}>('alerts');

	const categoryContext = getContext<{
		categories: CategoryState;
		reset: () => void;
		updateCategory: (data: CategorySummaryResponse) => void;
		toggleCategorySelected: (category: string) => void;
	}>('category');

	const fileProcessor = new FileProcessorService();
	let unsubscriber: Promise<UnlistenFn>[] = [];
	let isDeleting = $state<boolean>(false);
	let showDeleteConfirm = $state<boolean>(false);

	const scannerStatusHandler = (event: Event<string>) => {
		switch (event.payload) {
			case statusValues.SCAN_START:
				analyzerContext.setStatus('scanning');
				analyzerContext.updateUI('Scanning osu! Directory...', 'Found 0 files');
				break;
			case statusValues.PARSE_START:
				analyzerContext.setStatus('parsing');
				analyzerContext.updateUI('Parsing Files...', 'Parsed 0 files');
				break;
			case statusValues.FILTER_START:
				analyzerContext.setStatus('filtering');
				analyzerContext.updateUI('Filtering Files...', '');
				break;
			case statusValues.SCAN_CANCELLED ||
				statusValues.PARSE_CANCELLED ||
				statusValues.FILTER_CANCELLED:
				alertContext.show({
					type: 'warning',
					title: 'Cancelled',
					message: 'Analysis was cancelled'
				});
				break;
			case statusValues.DELETION_CANCELLED:
				isDeleting = false;
				showDeleteConfirm = false;
				fileProcessor.getCategorySummary().then((summary) => {
					categoryContext.updateCategory(summary);
					analyzerContext.setSummary(summary);
				});
				alertContext.show({
					type: 'warning',
					title: 'Cancelled',
					message: 'Deletion was cancelled'
				});
				break;
		}
	};

	const scannerScanCountsHandler = (event: Event<number>) => {
		analyzerContext.updateScanCount(event.payload);
	};

	const scannerParseCountsHandler = (event: Event<number>) => {
		analyzerContext.updateParseCount(event.payload);
	};

	const scannerFilterCountsHandler = (event: Event<CounterUpdate>) => {
		analyzerContext.updateFilterCounts(event.payload);
	};

	$effect(() => {
		unsubscriber = [
			listen<string>(file_processor.STATUS, scannerStatusHandler),
			listen<number>(file_processor.SCAN_COUNTS, scannerScanCountsHandler),
			listen<number>(file_processor.PARSE_COUNTS, scannerParseCountsHandler),
			listen<CounterUpdate>(file_processor.FILTER_COUNTS, scannerFilterCountsHandler)
		];

		return () => {
			unsubscriber.forEach((unsub) => unsub.then((fn) => fn()));
		};
	});

	const openFolderDialog = async () => {
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				defaultPath: selectedDirectory || undefined
			});

			selectedDirectory = selected ? selected : selectedDirectory;
			analyzerContext.reset();
		} catch (err) {
			alertContext.show({
				type: 'error',
				title: 'Error',
				message: 'Unable to open the folder dialog'
			});
		}
	};

	const cancelOperation = async () => {
		try {
			analyzerContext.setStatus('cancelling');
			analyzerContext.updateUI('Cancelling...', '');
			await fileProcessor.cancelOperation();
		} catch (err) {
			alertContext.show({
				type: 'error',
				title: 'Error',
				message: 'Failed to cancel operation'
			});
		}
	};

	const startAnalysis = async () => {
		if (!selectedDirectory) {
			alertContext.show({
				type: 'error',
				title: 'Error',
				message: 'Please select a directory first'
			});
			return;
		}

		try {
			await fileProcessor.scanDirectory(selectedDirectory);
			let categorySummary = await fileProcessor.getCategorySummary();
			if (analyzerContext.analyzer.status === 'cancelling') {
				analyzerContext.setStatus('idle');
				analyzerContext.reset();
				return;
			}
			categoryContext.updateCategory(categorySummary);
			analyzerContext.setSummary(categorySummary);
			analyzerContext.setStatus('complete');
		} catch (err) {
			alertContext.show({
				type: 'error',
				title: 'Error',
				message: 'Failed to analyze directory'
			});
			analyzerContext.setStatus('idle');
		}
	};

	const handleDeletion = async (selectedCategories: string[]) => {
		try {
			isDeleting = true;
			await fileProcessor.deleteFiles(selectedCategories);
			isDeleting = false;
		} catch (err) {
			alertContext.show({
				type: 'error',
				title: 'Error',
				message: 'Failed to delete files'
			});
		}
	};
</script>

<div class="space-y-4">
	<Card.Root>
		<Card.Header>
			<Card.Title class="flex items-center gap-2">
				<FolderOpen class="h-5 w-5" />
				Select osu! Directory
			</Card.Title>
		</Card.Header>
		<Card.Content>
			<div class="flex items-center gap-4">
				<Input
					type="text"
					placeholder="osu! Directory"
					class="border-zinc-700"
					disabled={true}
					value={selectedDirectory}
				/>
				{#if analyzerContext.analyzer.status === 'idle' || analyzerContext.analyzer.status === 'complete'}
					<Button on:click={openFolderDialog}>Browse</Button>
				{:else if analyzerContext.analyzer.status === 'scanning' || analyzerContext.analyzer.status === 'parsing' || analyzerContext.analyzer.status === 'filtering' || analyzerContext.analyzer.status === 'cancelling'}
					<Button
						on:click={cancelOperation}
						disabled={analyzerContext.analyzer.status === 'cancelling'}>Cancel</Button
					>
				{/if}
			</div>
		</Card.Content>
	</Card.Root>
	{#if analyzerContext.analyzer.status === 'idle'}
		<Button
			on:click={startAnalysis}
			size="lg"
			class="flex w-full items-center justify-center gap-2 rounded-lg px-6 py-4"
		>
			<Search class="h-5 w-5" />
			Analyze osu! Directory
		</Button>
	{:else if analyzerContext.analyzer.status === 'scanning' || analyzerContext.analyzer.status === 'parsing' || analyzerContext.analyzer.status === 'filtering' || analyzerContext.analyzer.status === 'cancelling'}
		<Analyzer
			title={analyzerContext.analyzer.ui.analyzer.title}
			subtitle={analyzerContext.analyzer.ui.analyzer.subtitle}
			counts={analyzerContext.analyzer.counts.filtered}
		/>
	{:else if analyzerContext.analyzer.status === 'complete'}
		<Summary
			data={analyzerContext.analyzer.summary}
			bind:showDeleteConfirm
			bind:isDeleting
			{onCategoryExpandClick}
			onDelete={handleDeletion}
			onDeleteComplete={() => {
				analyzerContext.reset();
				selectedDirectory = '';
			}}
		/>
	{/if}
</div>
