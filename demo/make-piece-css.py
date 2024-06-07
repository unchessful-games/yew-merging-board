import base64
import os

anims = {
    'white': '''<style>
@keyframes fade {
    0% {
        fill: #ffaaaa;
    }
    50% {
        fill: #ff0000;
    }
    100% {
        fill: #ffaaaa;
    }
}</style>
''',
    'black': '''<style>
@keyframes fade {
    0% {
        fill: #00aa00;
    }
    50% {
        fill: #00ff00;
    }
    100% {
        fill: #00aa00;
    }
}</style>
''',
}

with open('./css/cburnett.css', 'w') as f:
    for piece_file in sorted(os.listdir('./assets')):
        if piece_file.endswith('.svg'):
            color = piece_file.split('-')[0]
            piece_words = piece_file.split('.')[0].split('-')[1:]
            piece = '-'.join(piece_words)
            with open(f'./assets/{piece_file}', 'rb') as svg:
                text = svg.read().decode('utf-8')
                # If it's a unitary piece, then add an animation for the "main" ID
                if len(piece_words) == 1:
                    text_anim = text.replace('</svg>',
                                                anims[color] + \
                                                '<style>#main { animation: fade 1s infinite; } #main * { animation: fade 1s infinite; }</style>' + \
                                                '</svg>')
                    data = base64.b64encode(text.encode('utf-8')).decode('ascii')
                    print(f'.cg-wrap piece.{color}.{piece} ' + '{', file=f)
                    print(f"    background-image: url('data:image/svg+xml;base64,{data}');", file=f)
                    print('}', file=f)
                    anim_data = base64.b64encode(text_anim.encode('utf-8')).decode('ascii')
                    print(f'.cg-wrap piece.{color}.{piece}.selected ' + '{', file=f)
                    print(f"    background-image: url('data:image/svg+xml;base64,{anim_data}');", file=f)
                    print('}', file=f)
                else:
                    # If it's a combo piece, add animations for "left" and "right" as well.
                    data = base64.b64encode(text.encode('utf-8')).decode('ascii')
                    print(f'.cg-wrap piece.{color}.{piece} ' + '{', file=f)
                    print(f"    background-image: url('data:image/svg+xml;base64,{data}');", file=f)
                    print('}', file=f)

                    text_anim_left = text.replace('</svg>',
                                                anims[color] + \
                                                '<style>#left { animation: fade 1s infinite; } #left * { animation: fade 1s infinite; }</style>' + \
                                                '</svg>')
                    anim_data_left = base64.b64encode(text_anim_left.encode('utf-8')).decode('ascii')
                    print(f'.cg-wrap piece.{color}.{piece}.selected.left ' + '{', file=f)
                    print(f"    background-image: url('data:image/svg+xml;base64,{anim_data_left}');", file=f)
                    print('}', file=f)

                    text_anim_right = text.replace('</svg>',
                                                anims[color] + \
                                                '<style>#right { animation: fade 1s infinite; } #right * { animation: fade 1s infinite; }</style>' + \
                                                '</svg>')
                    anim_data_right = base64.b64encode(text_anim_right.encode('utf-8')).decode('ascii')
                    print(f'.cg-wrap piece.{color}.{piece}.selected.right ' + '{', file=f)
                    print(f"    background-image: url('data:image/svg+xml;base64,{anim_data_right}');", file=f)
                    print('}', file=f)

                    text_anim_full = text.replace('</svg>',
                                                anims[color] + \
                                                '<style>#left { animation: fade 1s infinite; } #left * { animation: fade 1s infinite; } #right { animation: fade 1s infinite; } #right * { animation: fade 1s infinite; }</style>' + \
                                                '</svg>')
                    anim_data_full = base64.b64encode(text_anim_full.encode('utf-8')).decode('ascii')
                    print(f'.cg-wrap piece.{color}.{piece}.selected.full ' + '{', file=f)
                    print(f"    background-image: url('data:image/svg+xml;base64,{anim_data_full}');", file=f)
                    print('}', file=f)

