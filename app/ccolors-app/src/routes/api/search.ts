import { ColorFamily } from "$lib/_colors";

export async function get({ query }) {

    if (!query.has('text')) {
        return {
            status: 500
        }
    }

    const searchText: string = query.get('text');
    const textSplit: string[] = searchText.split(' ');

    const params = new URLSearchParams();

    params.set('name', searchText);

    const hexCode = textSplit.find(element => element.startsWith('#'));

    // Only support full hex codes for now, no single character shorthand
    if (hexCode !== undefined && hexCode.length === 7) {
        params.set('hex', hexCode.replace('#', ''))
    }

    const colorFamilyStrings: string[] = Object.values(ColorFamily).filter(val => typeof val === 'string');

    const colorFamily: string = textSplit.find(element => colorFamilyStrings.includes(element.toLocaleUpperCase()))

    if (colorFamily !== undefined) {
        params.set('color_family', colorFamily.toLocaleLowerCase())
    }

    const url = "http://localhost:8000/colors/search?" + params.toString();

    console.log(url);

    const response = await fetch(url);

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