import { uniq } from 'lodash';
import { inject } from 'vue';
import type { Entry } from "./jmdict";
import { Character } from './kanjidic';
import { Cache } from './cache'
import { Kanji } from './kanjivg';

interface BasicSearchResult {
	kanji: Character[],
	kanjivg: Kanji[],
	results: Entry[],
	resultsTotal: number,
	time: string,
}

export interface SearchResult extends BasicSearchResult {
	// These properties are used by "searchMore"
	searchTerm: string,
	lastPageSize: number,
	clientTime: string
}

export class SearchService {
	private readonly cache = new Cache();

	constructor(public baseUrl = '/api') {
		this.cache.disabled = true;
	}

	async search(searchTerm: string, params = {} as { skip?: number, take?: number }) {
		const start = performance.now();
		const result = await this.get<BasicSearchResult>('/search', { searchTerm: searchTerm.trim(), ...params })
		const end = performance.now();

		return {
			searchTerm: searchTerm.toLowerCase(),
			lastPageSize: params.take,
			clientTime: `${end - start}ms`,
			...result
		} as SearchResult;
	}
	async searchMore(searchResult: SearchResult, take?: number) {
		const moreResults = await this.search(
			searchResult.searchTerm,
			{
				skip: searchResult.results.length,
				take: take ?? searchResult.lastPageSize
			}
		);
		moreResults.results.unshift(...searchResult.results);
		return moreResults;
	}
	async searchKanji(searchTerm: string) {
		const results = await this.search(searchTerm, {  });
		const kanji = uniq(
			results.results
			.flatMap(r => r.kanji!)
			.filter(k => k != undefined)
			.map(k => k.value)
			.flatMap(text => text.split(''))
			.filter(k => k.charCodeAt(0) >= 0x4e00 && k.charCodeAt(0) < 0x9FBF)
		);
		return kanji;
	}

	private get<T>(url: string, params: { [key: string]: string|number }) {
		let completeURL = `${this.baseUrl}${url}`;
		if(params && Object.keys(params).length > 0) {
			completeURL += '?' + Object.entries(params)
				.filter(x => x[1] !== undefined && x[1] !== null)
				.map(x => `${encodeURIComponent(x[0])}=${encodeURIComponent(x[1])}`)
				.join('&')
		}

		return this.cache.getOrCreate(
			completeURL,
			() => fetch(completeURL).then(r => r.json() as Promise<T>)
		);
	}
}

export function useSearch() {
	return inject('search-service') as SearchService;
}
