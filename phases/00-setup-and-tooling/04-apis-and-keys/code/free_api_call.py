from huggingface_hub import InferenceClient
import os

client = InferenceClient(
  model= "meta-llama/Llama-3.3-70B-Instruct",
  token= os.environ.get("HUGGINGFACE_TOKEN")
)

resp = client.chat.completions.create(
  messages=[{"role":"user", "content":"What is a neural network"}]
)

print(resp.choices[0].message.content)