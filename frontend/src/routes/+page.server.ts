import { getFaithDailyStats, getFaithWeeklyStats, getBibleStats } from '$lib/api/client';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const [dailyStats, weeklyStats, bibleStats] = await Promise.all([
		getFaithDailyStats(),
		getFaithWeeklyStats(),
		getBibleStats()
	]);

	return {
		dailyStats,
		weeklyStats,
		bibleStats
	};
};
