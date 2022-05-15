# Pythonでシーザー暗号に変換する関数
def encrypt(text, shift):
    # 'A'と'Z'の文字コードを得る
    code_a = ord('A')
    code_z = ord('Z')
    # 結果を代入する変数を用意
    result = ""
    # 一文字ずつ繰り返す
    for ch in text:
        # 文字コードに変換
        code = ord(ch)
        # AからZの間か？
