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
	import type { DailyStats } from '$lib/api/client';
	import chartColors from '$lib/theme/chartColors';

	// Register Chart.js components
	ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

	interface Props {
		data: DailyStats;
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

	// Transform data for Chart.js format
	const chartData = $derived({
		labels: data.days.map((day) => formatDate(day.date)),
		datasets: [
			{
				label: 'Minutes Studied',
				data: data.days.map((day) => day.minutes),
				backgroundColor: data.days.map((day) =>
					isSunday(day.date) ? chartColors.bar.background.red : chartColors.bar.background.blue
				),
				borderColor: data.days.map((day) =>
					isSunday(day.date) ? chartColors.bar.border.red : chartColors.bar.border.blue
				),
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: data.days.map((day) =>
					isSunday(day.date) ? chartColors.bar.hover.red : chartColors.bar.hover.blue
				)
			}
		]
	});

	const options = {
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			legend: {
				display: false
			},
			tooltip: {
				callbacks: {
					label: (context: { parsed: { y: number | null } }) => {
						const value = context.parsed.y ?? 0;
						return `${value.toFixed(1)} minutes`;
					}
				}
			}
		},
		scales: {
			x: {
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
						// Color Sunday labels red to match their bars
						return isSunday(data.days[context.index].date)
							? chartColors.label.red
							: chartColors.label.gray;
					}
				}
			},
			y: {
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
