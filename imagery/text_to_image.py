import torch
from diffusers import StableDiffusionImg2ImgPipeline
from PIL import Image

pipe = StableDiffusionImg2ImgPipeline.from_pretrained(
    "CompVis/stable-diffusion-v1-4",
    revision="fp16",
    torch_dtype=torch.float16,
    use_auth_token=True,
)
pipe = pipe.to("cuda")

#  lms = LMSDiscreteScheduler(beta_start=0.00085, beta_end=0.012, beta_schedule="scaled_linear")
#  pipe.scheduler = lms

init_image = Image.open("/mnt/e/images/ai-generated/input-seattle.jpg")

prompt = "vector, snow-capped mountain with a sun rising above it, space needle, colorful buildings, landscape"
pipe.enable_attention_slicing()
with torch.autocast("cuda"):
    for i in range(5):
        pipe(
            prompt=prompt, init_image=init_image, strength=0.85, guidance_scale=7.5
        ).images[i].save(f"result-{i}.png")
