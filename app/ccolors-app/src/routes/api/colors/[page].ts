import {
    colorsPerPage, totalColors, totalPages
} from "$lib/stores/state";

export async function get({ params }) {
    const requestedPage = parseInt(params.page);

    let colors: number, limit: number;

    totalColors.subscribe(val => colors = val)();
    colorsPerPage.subscribe(val => limit = val)();

    if (colors !== undefined && limit !== undefined) {
        const pages = Math.ceil(colors / limit);
        totalPages.set(pages);
        const startAt = requestedPage + ((requestedPage - 1) * (limit - 1));
        
        const colors_url = 'http://localhost:8000/colors?limit=25&start_at=' + startAt;
        const response = await fetch(colors_url);

        if (response.ok) {
            return {
                body: {
                    json: await response.json()
                }
            };
        } else {
            return {
                status: response.status,
                error: new Error(`Error getting color list from API ${await response.json()}`)
            };
        }
    }
}