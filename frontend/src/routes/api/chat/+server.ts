import { env } from '$env/dynamic/private';
import { json, type RequestHandler } from '@sveltejs/kit';

type ChatMessage = {
	role: 'system' | 'user' | 'assistant';
	content: string;
};

export const POST: RequestHandler = async ({ request }) => {
	const apiKey = env.GROQ_API_KEY;
	if (!apiKey) {
		return json({ error: 'Missing GROQ_API_KEY' }, { status: 500 });
	}

	const model = (env.GROQ_MODEL && env.GROQ_MODEL.trim()) || 'llama-3.1-8b-instant';

	let body: unknown;
	try {
		body = await request.json();
	} catch {
		return json({ error: 'Invalid JSON body' }, { status: 400 });
	}

	const message = (body as { message?: unknown })?.message;
	if (typeof message !== 'string' || message.trim().length === 0) {
		return json({ error: 'Missing message' }, { status: 400 });
	}

	const trimmed = message.trim().slice(0, 2000);

	const messages: ChatMessage[] = [
		{
			role: 'system',
			content:
				'Eres un asistente para un ecommerce de tecnología llamado KronosTech. Ayudas a elegir productos, resolver dudas de compra/envíos/garantía, y recomiendas de forma breve y clara. Si falta información, haz 1-2 preguntas concretas.'
		},
		{ role: 'user', content: trimmed }
	];

	try {
		const resp = await fetch('https://api.groq.com/openai/v1/chat/completions', {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${apiKey}`,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				model,
				messages,
				temperature: 0.4,
				max_tokens: 300
			})
		});

		if (!resp.ok) {
			const raw = await resp.text().catch(() => '');
			let details: unknown = raw;
			try {
				details = raw ? JSON.parse(raw) : raw;
			} catch {
				// ignore
			}

			const maybeMessage =
				typeof details === 'object' && details
					? (details as { error?: { message?: string } }).error?.message
					: undefined;

			return json(
				{
					error: maybeMessage || 'Groq request failed',
					status: resp.status,
					details
				},
				{ status: 502 }
			);
		}

		const data = (await resp.json()) as {
			choices?: Array<{ message?: { content?: string } }>;
		};

		const content = data?.choices?.[0]?.message?.content;
		if (typeof content !== 'string' || content.trim().length === 0) {
			return json({ error: 'Empty response from Groq' }, { status: 502 });
		}

		return json({ reply: content.trim() });
	} catch (err) {
		return json({ error: 'Unexpected error', details: String(err) }, { status: 500 });
	}
};
