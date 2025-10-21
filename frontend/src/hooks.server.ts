import { env } from '$env/dynamic/private';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const authUsername = env.BASIC_AUTH_USERNAME;
	const authPassword = env.BASIC_AUTH_PASSWORD;

	// Fail if credentials are not configured
	if (!authUsername || !authPassword) {
		return new Response('Authentication not configured', {
			status: 500
		});
	}

	const authHeader = event.request.headers.get('Authorization');

	if (!authHeader || !authHeader.startsWith('Basic ')) {
		return new Response('Unauthorized', {
			status: 401,
			headers: {
				'WWW-Authenticate': 'Basic realm="Secure Area"'
			}
		});
	}

	// Decode base64 credentials
	const base64Credentials = authHeader.slice(6);
	const credentials = atob(base64Credentials);
	const [username, password] = credentials.split(':');

	// Check credentials
	if (username !== authUsername || password !== authPassword) {
		return new Response('Unauthorized', {
			status: 401,
			headers: {
				'WWW-Authenticate': 'Basic realm="Secure Area"'
			}
		});
	}

	// Authentication successful, continue with request
	return resolve(event);
};
