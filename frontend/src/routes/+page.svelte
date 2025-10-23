<script lang="ts">
	import Card from '$lib/components/Card.svelte';
	import DailyStudyChart from '$lib/components/DailyStudyChart.svelte';
	import WeeklyStudyChart from '$lib/components/WeeklyStudyChart.svelte';
	import WeeklyChurchChart from '$lib/components/WeeklyChurchChart.svelte';
	import BookStatsChart from '$lib/components/BookStatsChart.svelte';
	import AnkiDailyLearningProgressChart from '$lib/components/AnkiDailyLearningProgressChart.svelte';
	import AnkiWeeklyLearningProgressChart from '$lib/components/AnkiWeeklyLearningProgressChart.svelte';

	const { data } = $props();
</script>

<div class="container mx-auto p-8">
	<h1 class="mb-8 text-3xl font-bold">Faith Statistics</h1>

	<div class="grid grid-cols-1 gap-8 xl:grid-cols-2">
		<Card title="Daily Study Time (Last 30 Days)">
			<DailyStudyChart data={data.dailyStats} />
		</Card>

		<Card title="Weekly Study Time (Last 12 Weeks)">
			<WeeklyStudyChart data={data.weeklyStats} />
		</Card>

		<Card title="Time at Church (Last 12 Weeks)">
			<WeeklyChurchChart data={data.weeklyStats} />
		</Card>

		<div class="xl:col-span-2">
			<Card title="Bible Books Progress">
				<BookStatsChart data={data.bibleStats} />
			</Card>
		</div>

		<Card title="Learning Progress (Last 30 Days)">
			<AnkiDailyLearningProgressChart data={data.dailyStats} />
		</Card>

		<Card title="Learning Progress (Last 12 Weeks)">
			<AnkiWeeklyLearningProgressChart data={data.weeklyStats} />
		</Card>
	</div>
</div>
