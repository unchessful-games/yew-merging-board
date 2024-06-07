import base64
import os

with open('./css/cburnett.css', 'w') as f:
    for piece_file in os.listdir('./assets'):
        if piece_file.endswith('.svg'):
            color = piece_file.split('-')[0]
            piece = piece_file.split('.')[0].split('-')[1:]
            piece = '-'.join(piece)
            with open(f'./assets/{piece_file}', 'rb') as svg:
                data = base64.b64encode(svg.read()).decode('ascii')
                print(f'.cg-wrap piece.{color}.{piece}' + '{', file=f)
                print(f"    background-image: url('data:image/svg+xml;base64,{data}');", file=f)
                print('}', file=f)

