function my_function(){
  // fetch from URL
const API_URL = "https://simages.info"

// From url
const body = {url: URL_TO_IMAGE, num_similar: NUM_SIMILAR}

// From text with metadata
const body = {text: TEXT_QUERY, num_similar: NUM_SIMILAR, get_labels: "coco_yolov7"}

// From image
const body = {image: IMAGE_IN_BASE64, num_similar: NUM_SIMILAR}

const response = await axios.post(API_URL, body, {
            headers: {
                "Content-Type": "application/json",
                "x-api-key": X_API_KEY,
            },
            timeout: 300 * 1000,
        });
        return JSON.parse(response.data.body);
}