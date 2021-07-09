export async function get({ params }) {
    const metadataUrl = "http://localhost:8000/colors/info";
    const response = await fetch(metadataUrl);

    const json = await response.json();

    if (response.ok) {
        return {
            body: {
                json
            }
        }
    } else {
        return {
			status: response.status,
			error: new Error(`Error getting color list from API ${json}`)
		};
    }
}