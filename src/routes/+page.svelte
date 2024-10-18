<script lang="ts">
	import { setContext } from 'svelte';

	import MainView from '$lib/components/views/main.svelte';
	import CategoryView from '$lib/components/views/category.svelte';
	import CustomAlert from '$lib/components/ui/custom-alert/alert.svelte';
	import createAlertState from '$lib/states/alert.svelte.ts';
	import createAnalyzerState from '$lib/states/analyzer.svelte';
	import createCategoryState from '$lib/states/category.svelte';
	import type { CategoryUI } from '$lib/utils/interfaces.ts';

	let currentView = $state<string>('main');
	const categoryContext = createCategoryState();
	setContext('category', categoryContext);

	const alertContext = createAlertState();
	setContext('alerts', alertContext);

	const analyzerContext = createAnalyzerState();
	setContext('analyzer', analyzerContext);

	const setCurrentView = (view: string) => {
		currentView = view;
	};

	let selectedDirectory = $state<string>('');
	let selectedCategory = $state<CategoryUI | null>(null);
	const onCategoryExpandClick = (category: string) => {
		selectedCategory = categoryContext.categories[category];
		setCurrentView('category');
	};
</script>

<!-- Alert Container -->
<CustomAlert />
<div class="min-h-screen p-6">
	<div class="mx-auto max-w-4xl">
		<!-- Global Header -->
		<div class="mb-8 text-center">
			<h1 class="mb-2 text-4xl font-bold">osu!cleaner</h1>
			<p class="text-gray-400">Free up space by removing unnecessary osu! files</p>
		</div>
		{#if currentView === 'main'}
			<!-- View Container -->
			<MainView bind:selectedDirectory {onCategoryExpandClick} />
		{:else}
			<!-- Category View -->
			<CategoryView {selectedCategory} {setCurrentView} />
		{/if}
	</div>
</div>
