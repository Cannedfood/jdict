import type { Entry } from "./jmdict";

function delay(millis: number) {
    return new Promise((resolve) => setTimeout(resolve, millis));
}

const words = [
    "fuck",
    "piss off",
    "bugger off",
    "bloody hell",
    "bastard",
    "wanker",
    "bollocks",
]

export class SearchService {
    constructor(public baseUrl = '/api') {}

    fullSearch(searchTerm: string) {
        return this.get<Entry[]>('/search', { searchTerm });
    }

    async autocomplete(term: string) {
        if(term.length == 0)
            return [];

        await delay(1000);

        term = term.toLowerCase();
        return (
            words
            .filter(w => w.includes(term))
            .sort(w => Number(w.startsWith(term)))
        )
    }

    private get<T>(url: string, params: { [key: string]: string }) {
        let completeURL = `${this.baseUrl}${encodeURIComponent(url)}`;
        if(params && Object.keys(params).length > 0) {
            completeURL = '?' + Object.entries(params)
                .map(x => `${encodeURIComponent(x[0])}=${encodeURIComponent(x[1])}`)
                .join('&')
        }

        return fetch(completeURL).then(r => r.json() as Promise<T>);
    }
}