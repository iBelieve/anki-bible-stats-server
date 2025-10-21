<script lang="ts">
	import { Bar } from 'svelte5-chartjs';
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		BarElement,
		CategoryScale,
		LinearScale,
		type ScriptableScaleContext
	} from 'chart.js';
	import { Temporal } from '@js-temporal/polyfill';
	import type { FaithDailyStats } from '$lib/api/client';
	import chartColors from '$lib/theme/chartColors';

	// Register Chart.js components
	ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

	interface Props {
		data: FaithDailyStats;
	}

	const { data }: Props = $props();

	// Format date to show only month/day to prevent label overlap
	const formatDate = (dateStr: string) => {
		const date = Temporal.PlainDate.from(dateStr);
		return `${date.month}/${date.day}`;
	};

	// Check if a date is Sunday (dayOfWeek 7 = Sunday in Temporal)
	const isSunday = (dateStr: string) => {
		const date = Temporal.PlainDate.from(dateStr);
		return date.dayOfWeek === 7;
	};

	// Transform data for Chart.js format (stacked bar chart)
	const chartData = $derived({
		labels: data.days.map((day) => formatDate(day.date)),
		datasets: [
			// Bottom stack: Reading (Bible) in green
			{
				label: 'Reading',
				data: data.days.map((day) => day.reading_minutes),
				backgroundColor: chartColors.bar.background.green,
				borderColor: data.days.map((day) =>
					isSunday(day.date) ? chartColors.bar.border.red : chartColors.bar.border.green
				),
				borderWidth: data.days.map((day) => (isSunday(day.date) ? 2 : 1)),
				borderRadius: 4,
				hoverBackgroundColor: chartColors.bar.hover.green,
				stack: 'stack0'
			},
			// Top stack: Anki (Memorization) in blue
			{
				label: 'Anki',
				data: data.days.map((day) => day.anki_minutes),
				backgroundColor: chartColors.bar.background.blue,
				borderColor: data.days.map((day) =>
					isSunday(day.date) ? chartColors.bar.border.red : chartColors.bar.border.blue
				),
				borderWidth: data.days.map((day) => (isSunday(day.date) ? 2 : 1)),
				borderRadius: 4,
				hoverBackgroundColor: chartColors.bar.hover.blue,
				stack: 'stack0'
			}
		]
	});

	const options = {
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			legend: {
				display: true,
				position: 'top' as const
			},
			tooltip: {
				callbacks: {
					label: (context: { dataset: { label?: string }; parsed: { y: number | null } }) => {
						const value = context.parsed.y ?? 0;
						const label = context.dataset.label || '';
						return `${label}: ${value.toFixed(1)} min`;
					},
					footer: (tooltipItems: Array<{ parsed: { y: number | null } }>) => {
						const total = tooltipItems.reduce((sum, item) => sum + (item.parsed.y ?? 0), 0);
						return `Total: ${total.toFixed(1)} min`;
					}
				}
			}
		},
		scales: {
			x: {
				stacked: true,
				grid: {
					display: false
				},
				ticks: {
					maxRotation: 45,
					minRotation: 45,
					font: {
						size: 11
					},
					color: (context: ScriptableScaleContext) => {
						// Color Sunday labels red
						return isSunday(data.days[context.index].date)
							? chartColors.label.red
							: chartColors.label.gray;
					}
				}
			},
			y: {
				stacked: true,
				beginAtZero: true,
				grid: {
					color: chartColors.grid.gray
				},
				title: {
					display: true,
					text: 'Minutes'
				}
			}
		}
	};
</script>

<div class="h-64 w-full md:h-96">
	<Bar data={chartData} {options} />
</div>
