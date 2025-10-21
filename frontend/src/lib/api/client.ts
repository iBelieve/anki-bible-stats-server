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
export type HealthCheck = components['schemas']['HealthCheck'];
export type FaithTodayStats = components['schemas']['FaithTodayStats'];
export type FaithDailyStats = components['schemas']['FaithDailyStats'];
export type FaithDayStats = components['schemas']['FaithDayStats'];
export type FaithDailySummary = components['schemas']['FaithDailySummary'];
export type FaithWeeklyStats = components['schemas']['FaithWeeklyStats'];
export type FaithWeekStats = components['schemas']['FaithWeekStats'];
export type FaithWeeklySummary = components['schemas']['FaithWeeklySummary'];

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
 * Get today's unified faith statistics, combining Bible reading and Anki memorization.
 */
export async function getFaithTodayStats(): Promise<FaithTodayStats> {
	const { data, error, response } = await apiClient.GET('/api/faith/today');

	if (error) {
		throw new Error(`Failed to fetch faith today stats: ${error.error}`);
	} else if (response.status === 401) {
		throw new Error('Unauthorized: Invalid or missing API key.');
	}

	return data!;
}

/**
 * Get unified faith statistics for the last 12 weeks, combining Bible reading and Anki memorization.
 */
export async function getFaithWeeklyStats(): Promise<FaithWeeklyStats> {
	const { data, error, response } = await apiClient.GET('/api/faith/weekly');

	if (error) {
		throw new Error(`Failed to fetch faith weekly stats: ${error.error}`);
	} else if (response.status === 401) {
		throw new Error('Unauthorized: Invalid or missing API key.');
	}

	return data!;
}

/**
 * Get unified faith statistics for the last 30 days, combining Bible reading and Anki memorization.
 */
export async function getFaithDailyStats(): Promise<FaithDailyStats> {
	const { data, error, response } = await apiClient.GET('/api/faith/daily');

	if (error) {
		throw new Error(`Failed to fetch faith daily stats: ${error.error}`);
	} else if (response.status === 401) {
		throw new Error('Unauthorized: Invalid or missing API key.');
	}

	return data!;
}
