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
	import type { FaithWeeklyStats } from '$lib/api/client';
	import chartColors from '$lib/theme/chartColors';
	import { formatMinutesToHoursMinutes } from '$lib/utils/timeFormat';

	// Register Chart.js components
	ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

	interface Props {
		data: FaithWeeklyStats;
	}

	const { data }: Props = $props();

	// Format date to show month/day for week start
	const formatDate = (dateStr: string) => {
		const date = Temporal.PlainDate.from(dateStr);
		return `${date.month}/${date.day}`;
	};

	// Transform data for Chart.js format (stacked bar chart)
	const chartData = $derived({
		labels: data.weeks.map((week) => formatDate(week.week_start)),
		datasets: [
			// Bottom stack: Reading (Bible) in green
			{
				label: 'Reading',
				data: data.weeks.map((week) => week.reading_minutes),
				backgroundColor: chartColors.bar.background.green,
				borderColor: chartColors.bar.border.green,
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: chartColors.bar.hover.green,
				stack: 'stack0'
			},
			// Middle stack: Prayer in purple
			{
				label: 'Prayer',
				data: data.weeks.map((week) => week.prayer_minutes),
				backgroundColor: chartColors.bar.background.purple,
				borderColor: chartColors.bar.border.purple,
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: chartColors.bar.hover.purple,
				stack: 'stack0'
			},
			// Top stack: Anki (Memorization) in blue
			{
				label: 'Anki',
				data: data.weeks.map((week) => week.anki_minutes),
				backgroundColor: chartColors.bar.background.blue,
				borderColor: chartColors.bar.border.blue,
				borderWidth: 1,
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
						const minutes = context.parsed.y ?? 0;
						const label = context.dataset.label || '';
						return `${label}: ${formatMinutesToHoursMinutes(minutes)}`;
					},
					footer: (tooltipItems: Array<{ parsed: { y: number | null } }>) => {
						const total = tooltipItems.reduce((sum, item) => sum + (item.parsed.y ?? 0), 0);
						return `Total: ${formatMinutesToHoursMinutes(total)}`;
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
					text: 'Hours'
				},
				ticks: {
					stepSize: 30, // Show ticks every 30 minutes (half hour)
					callback: function (value: string | number) {
						// Convert the tick value (minutes) to hours and minutes format
						const minutes = typeof value === 'number' ? value : parseFloat(value);
						// Only show labels on whole hours, hide labels for half hours
						if (minutes % 60 === 0) {
							return formatMinutesToHoursMinutes(minutes);
						}
						return ''; // Return empty string for half-hour ticks (shows tick but no label)
					}
				}
			}
		}
	};
</script>

<div class="h-64 w-full md:h-96">
	<Bar data={chartData} {options} />
</div>
