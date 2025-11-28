export type Emote = {
	name: string;
	url: string;
};

type EmoteSet = {
	emotes: {
		name: string;
		data: {
			host: {
				url: string;
			};
		};
	}[];
};

export class Emotes {
	emotes: Emote[] = [];

	constructor() {
		fetch('https://7tv.io/v3/emote-sets/global')
			.then(r => r.json())
			.then(r => {
				this.addEmoteSet(r);
			});

		fetch('https://7tv.io/v3/users/twitch/136877209')
			.then(r => r.json())
			.then(r => {
				this.addEmoteSet(r.emote_set);
			});
	}

	addEmoteSet(set: EmoteSet) {
		this.emotes.push(...set.emotes.map(e => ({
			name: e.name,
			url: `https:${e.data.host.url}/1x.avif`,
		})));
	}

	public getEmote(name: string): Emote | undefined {
		return this.emotes.find(e => e.name === name);
	}
}