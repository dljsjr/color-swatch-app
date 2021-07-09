export async function get({ params }) {
    
    const searchUrl = "http://localhost:8000/colors/search?&color_family=" + params.family;
    const response = await fetch(searchUrl);

    console.log(response);

    if (response.ok) {
        const retJson = await response.json();
        console.log(retJson);
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