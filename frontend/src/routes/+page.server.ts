import {
	getFaithDailyStats,
	getFaithWeeklyStats,
	getBibleStats,
	getTopPlaces
} from '$lib/api/client';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const [dailyStats, weeklyStats, bibleStats, topPlaces] = await Promise.all([
		getFaithDailyStats(),
		getFaithWeeklyStats(),
		getBibleStats(),
		getTopPlaces()
	]);

	return {
		dailyStats,
		weeklyStats,
		bibleStats,
		topPlaces
	};
};
