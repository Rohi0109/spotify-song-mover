'''doing this in python bc im lzay  '''
import json
import os

SPOTIFY_DIR = 'spotify_files'

def combine_jsons():
    """Combine all JSON files in the spotify_files directory."""
    combined_data = []
    
    for filename in os.listdir(SPOTIFY_DIR):
       
            filepath = os.path.join(SPOTIFY_DIR, filename)
            with open(filepath, 'r') as f:
                data = json.load(f)
                combined_data.extend(data if isinstance(data, list) else [data])
    
    # Save combined file
    output_path = os.path.join(SPOTIFY_DIR, 'combined.json')
    with open(output_path, 'w') as f:
        json.dump(combined_data, f, indent=2)

if __name__ == '__main__':
    combine_jsons()


