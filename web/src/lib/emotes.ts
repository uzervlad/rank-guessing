export type Emote = {
	name: string;
	url: string;
	zero_width: boolean; // TODO
};

export class Emotes {
	emotes: Emote[] = [];

	public init() {
		fetch('/api/emotes')
			.then(r => r.json())
			.then(r => {
				this.emotes.push(...r);
			});
	}

	public getEmote(name: string): Emote | undefined {
		return this.emotes.find(e => e.name === name);
	}
}