export async function get({ params }) {
    
    const search_url = "http://localhost:8000/colors/search?&color_family=" + params.family;
    const response = await fetch(search_url);

    console.log(response);

    if (response.ok) {
        const ret_json = await response.json();
        console.log(ret_json);
        return {
            body: {
                json: ret_json
            }
        };
    } else {
        return {
            status: response.status,
            error: new Error(`Error getting color list from API ${await response.json()}`)
        };
    }
}