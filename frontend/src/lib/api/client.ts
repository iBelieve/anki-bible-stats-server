import createClient from 'openapi-fetch';
import type { paths, components } from './schema';
import { env } from '$env/dynamic/private';

/**
 * Anki Bible Stats API client
 *
 * This client is configured for server-side use only and requires
 * the ANKISTATS_API_KEY environment variable to be set.
 */
const apiClient = createClient<paths>({
	baseUrl: env.LIFESTATS_BASE_URL,
	headers: {
		Authorization: `Bearer ${env.LIFESTATS_API_KEY}`
	}
});

// Type exports for convenience
export type BibleStats = components['schemas']['BibleStats'];
export type DailyStats = components['schemas']['DailyStats'];
export type WeeklyStats = components['schemas']['WeeklyStats'];
export type TodayStats = components['schemas']['TodayStats'];
export type HealthCheck = components['schemas']['HealthCheck'];

/**
 * Get Bible book statistics including Old and New Testament breakdowns.
 */
export async function getBibleStats(): Promise<BibleStats> {
	const { data, error, response } = await apiClient.GET('/api/anki/books');

	if (error) {
		throw new Error(`Failed to fetch Bible stats: ${error.error}`);
	} else if (response.status === 401) {
		throw new Error('Unauthorized: Invalid or missing API key.');
	}

	return data!;
}

/**
 * Get daily study time and progress for the last 30 days.
 */
export async function getDailyStats(): Promise<DailyStats> {
	const { data, error, response } = await apiClient.GET('/api/anki/daily');

	if (error) {
		throw new Error(`Failed to fetch daily stats: ${error.error}`);
	} else if (response.status === 401) {
		throw new Error('Unauthorized: Invalid or missing API key.');
	}

	return data!;
}

/**
 * Get today's study time.
 */
export async function getTodayStats(): Promise<TodayStats> {
	const { data, error, response } = await apiClient.GET('/api/anki/today');

	if (error) {
		throw new Error(`Failed to fetch today stats: ${error.error}`);
	} else if (response.status === 401) {
		throw new Error('Unauthorized: Invalid or missing API key.');
	}

	return data!;
}

/**
 * Get weekly study time and progress for the last 12 weeks.
 */
export async function getWeeklyStats(): Promise<WeeklyStats> {
	const { data, error, response } = await apiClient.GET('/api/anki/weekly');

	if (error) {
		throw new Error(`Failed to fetch weekly stats: ${error.error}`);
	} else if (response.status === 401) {
		throw new Error('Unauthorized: Invalid or missing API key.');
	}

	return data!;
}
