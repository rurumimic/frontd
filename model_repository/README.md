# Model Repository

## Install HuggingFace CLI

### Python venv

```bash
uv venv
source .venv/bin/activate
```

### Install HF Transfer

```bash
uv pip install "huggingface_hub[hf_transfer]"
```

---

## Download Models

### BAAI/bge-m3

- [BAAI/bge-m3](https://huggingface.co/BAAI/bge-m3)

```bash
mkdir -p BAAI/bge-m3/1
cd BAAI/bge-m3/1
```

```bash
HF_HUB_ENABLE_HF_TRANSFER=1 huggingface-cli download "BAAI/bge-m3" --local-dir bge-m3
```

---

## Models

```bash
      model_repository
      └── BAAI/bge-m3/1/bge-m3
          ├── 1_Pooling
  191     │   └── config.json
 2.1M     ├── colbert_linear.pt
  687     ├── config.json
  123     ├── config_sentence_transformers.json
          ├── imgs/
 127k     ├── long.jpg
  349     ├── modules.json
          ├── onnx/
  698     │   ├── config.json
  66k     │   ├── Constant_7_attr__value
 725k     │   ├── model.onnx
 2.3G     │   ├── model.onnx_data
 5.1M     │   ├── sentencepiece.bpe.model
  964     │   ├── special_tokens_map.json
  17M     │   ├── tokenizer.json
 1.2k     │   └── tokenizer_config.json
 2.3G     ├── pytorch_model.bin
  16k     ├── README.md
   54     ├── sentence_bert_config.json
 5.1M     ├── sentencepiece.bpe.model
 3.5k     ├── sparse_linear.pt
  964     ├── special_tokens_map.json
  17M     ├── tokenizer.json
  444     └── tokenizer_config.json
```

