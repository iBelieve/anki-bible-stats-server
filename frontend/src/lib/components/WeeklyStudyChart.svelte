<script lang="ts">
	import { Bar } from 'svelte5-chartjs';
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		BarElement,
		CategoryScale,
		LinearScale
	} from 'chart.js';
	import { Temporal } from '@js-temporal/polyfill';
	import type { WeeklyStats } from '$lib/api/client';
	import chartColors from '$lib/theme/chartColors';

	// Register Chart.js components
	ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

	interface Props {
		data: WeeklyStats;
	}

	const { data }: Props = $props();

	// Format date to show month/day for week start
	const formatDate = (dateStr: string) => {
		const date = Temporal.PlainDate.from(dateStr);
		return `${date.month}/${date.day}`;
	};

	// Transform data for Chart.js format
	const chartData = $derived({
		labels: data.weeks.map((week) => formatDate(week.week_start)),
		datasets: [
			{
				label: 'Minutes Studied',
				data: data.weeks.map((week) => week.minutes),
				backgroundColor: chartColors.bar.background.green,
				borderColor: chartColors.bar.border.green,
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: chartColors.bar.hover.green
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
