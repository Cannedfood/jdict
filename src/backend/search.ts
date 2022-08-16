import type { Entry } from "./jmdict";

interface BasicSearchResult {
	results: Entry[],
	resultsTotal: number,
	time: string,
}

export interface SearchResult extends BasicSearchResult {
	// These properties are used by "searchMore"
	searchTerm: string,
	lastPageSize: number,
}

export class SearchService {
	constructor(public baseUrl = '/api') {}

	async search(searchTerm: string, params = {} as { skip?: number, take?: number }) {
		return {
			searchTerm,
			lastPageSize: params.take,
			...await this.get<BasicSearchResult>('/search', { searchTerm, ...params })
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

	private get<T>(url: string, params: { [key: string]: string|number }) {
		let completeURL = `${this.baseUrl}${url}`;
		if(params && Object.keys(params).length > 0) {
			completeURL += '?' + Object.entries(params)
				.filter(x => x[1] !== undefined && x[1] !== null)
				.map(x => `${encodeURIComponent(x[0])}=${encodeURIComponent(x[1])}`)
				.join('&')
		}
		return fetch(completeURL).then(r => r.json() as Promise<T>);
	}
}
