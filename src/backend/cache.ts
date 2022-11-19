export class Cache {
	public disabled = false;
	public entries = {} as { [key: string]: any };

	public getOrCreate<T>(key: string, create: () => T) {
		if(this.disabled)
			return create();

		let entry = this.entries[key];
		if(entry)
			return entry;

		entry = create();
		this.entries[key] = entry;

		return entry;
	}
}
