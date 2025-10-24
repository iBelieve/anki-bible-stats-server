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
	import type { PlaceStats } from '$lib/api/client';
	import chartColors from '$lib/theme/chartColors';

	// Register Chart.js components
	ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

	interface Props {
		data: PlaceStats[];
	}

	const { data }: Props = $props();

	// Format hours to show hours and minutes
	const formatHours = (hours: number) => {
		const h = Math.floor(hours);
		const m = Math.round((hours - h) * 60);
		if (m === 0) {
			return `${h}h`;
		}
		return `${h}h ${m}m`;
	};

	// Transform data for Chart.js format (horizontal bar chart)
	const chartData = $derived({
		labels: data.map((place) => place.place_name),
		datasets: [
			{
				label: 'Hours',
				data: data.map((place) => place.hours),
				backgroundColor: chartColors.bar.background.orange,
				borderColor: chartColors.bar.border.orange,
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: chartColors.bar.hover.orange
			}
		]
	});

	const options = {
		indexAxis: 'y' as const, // Horizontal bars
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			legend: {
				display: false // Hide legend since there's only one dataset
			},
			tooltip: {
				callbacks: {
					label: (context: { parsed: { x: number | null } }) => {
						const hours = context.parsed.x ?? 0;
						return `Time: ${formatHours(hours)}`;
					}
				}
			}
		},
		scales: {
			x: {
				beginAtZero: true,
				grid: {
					color: chartColors.grid.gray
				},
				title: {
					display: true,
					text: 'Hours'
				},
				ticks: {
					callback: function (value: string | number) {
						const hours = typeof value === 'number' ? value : parseFloat(value);
						return formatHours(hours);
					}
				}
			},
			y: {
				grid: {
					display: false
				},
				ticks: {
					font: {
						size: 11
					}
				}
			}
		}
	};
</script>

<div class="h-64 w-full md:h-80">
	<Bar data={chartData} {options} />
</div>
