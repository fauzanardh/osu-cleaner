<script lang="ts">
	import { cn } from '$lib/utils';
	import { CategoryIconComponents } from '$lib/utils/icons';

	let {
		id,
		count,
		label,
		className = ''
	} = $props<{
		id: string;
		count: number;
		label: string;
		className?: string;
	}>();

	let Icon = CategoryIconComponents[id];
</script>

<div class={cn('flex flex-col items-center rounded-lg border border-zinc-800 p-2', className)}>
	<Icon class="mb-2 h-5 w-5" />
	<span class="text-2xl font-bold" role="status" aria-live="polite">
		{count.toLocaleString()}
	</span>
	<span class="text-xs text-gray-400">{label}</span>
</div>
