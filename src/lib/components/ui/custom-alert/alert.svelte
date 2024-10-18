<script lang="ts">
	import { AlertCircle, CheckCircle, Info, XCircle } from 'lucide-svelte';
	import { getContext } from 'svelte';
	import { quintOut } from 'svelte/easing';
	import { fly } from 'svelte/transition';

	import * as Alert from '$lib/components/ui/alert';
	import type { AlertMessage } from '$lib/utils/interfaces';

	const icons = {
		error: XCircle,
		success: CheckCircle,
		warning: AlertCircle,
		info: Info
	};

	const alertStyles = {
		error: {
			container: 'border-red-500 bg-red-500/15 text-red-500',
			description: 'text-red-400',
			icon: '!text-red-500'
		},
		success: {
			container: 'border-green-500 bg-green-500/15 text-green-500',
			description: 'text-green-400',
			icon: '!text-green-500'
		},
		warning: {
			container: 'border-yellow-500 bg-yellow-500/15 text-yellow-500',
			description: 'text-yellow-400',
			icon: '!text-yellow-500'
		},
		info: {
			container: 'border-blue-500 bg-blue-500/15 text-blue-500',
			description: 'text-blue-400',
			icon: '!text-blue-500'
		}
	} as const;

	const alertContext = getContext<{
		readonly alerts: AlertMessage[];
		show: (alert: Omit<AlertMessage, 'id'>) => void;
		dismiss: (id: string) => void;
		clear: () => void;
	}>('alerts');
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
	{#each alertContext.alerts as alert (alert.id)}
		<div
			class="w-full max-w-md"
			transition:fly={{ y: 30, duration: 300, easing: quintOut }}
			role="alert"
		>
			<Alert.Root class={`relative ${alertStyles[alert.type].container}`}>
				<svelte:component
					this={icons[alert.type]}
					class={`h-4 w-4 ${alertStyles[alert.type].icon}`}
				/>
				<Alert.Title>{alert.title}</Alert.Title>
				<Alert.Description class={alertStyles[alert.type].description}>
					{alert.message}
					{#if alert.details}
						<button class="text-sm underline" on:click={() => console.log(alert.details)}>
							Show details
						</button>
					{/if}
				</Alert.Description>

				{#if alert.persist}
					<button class="absolute right-2 top-2" on:click={() => alertContext.dismiss(alert.id)}>
						<XCircle class="h-4 w-4" />
					</button>
				{/if}
			</Alert.Root>
		</div>
	{/each}
</div>
