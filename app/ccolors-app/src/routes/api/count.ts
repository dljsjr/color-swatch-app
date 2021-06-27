export async function get({ params }) {
    const metadata_url = "http://api:8000/colors/info";
    const response = await fetch(metadata_url);

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