# GetECR CLI

## See releases for binaries

## Install with cargo
```bash
cargo install --git https://${GITHUB_TOKEN}:x-oauth-basic@github.com/cmac4603/getecr getecr
```

## Environment Variables
- `ECR_IMAGE_PREFIX` (if your images are collected with a common prefix, optionally add this env var)
- `ECR_REGISTRY_ID` (if not specified will use default from AWS)

### Get a single image tag & sha
```bash
$ getecr myapp 1.0.0
Image Tag: 1.0.0
Image Sha: sha256:f01292b8c692fd9a0367023dc5ab50fe4ced66efea0091e0c39475d8e94c098
```

### No repo
```bash
$ getecr noapp 1.0.0
Error: The repository with name 'noapp' does not exist in the registry with id '1234567890'
```

### No image tag found
```bash
$ getecr myapp 1.0.1
Error: No image tags found.
```
