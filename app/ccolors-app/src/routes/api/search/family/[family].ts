export async function get({ params }) {
    
    const searchUrl = "http://api:8000/colors/search?&color_family=" + params.family;
    const response = await fetch(searchUrl);

    if (response.ok) {
        const retJson = await response.json();

        return {
            body: {
                json: retJson
            }
        };
    } else {
        return {
            status: response.status,
            error: new Error(`Error getting color list from API ${await response.json()}`)
        };
    }
}