export async function get({ params }) {
    const requestedId = parseInt(params.id);

    const detailUrl = 'http://api:8000/colors/get?id=' + requestedId;
    const response = await fetch(detailUrl);

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