# Tokyo Explorer + AI
Explore Tokyo like never before. Powered by the Gemini API 2.0 EXP Pro and built in Rust, this app crafts immersive, AI-driven experiences—blending cultural insights, tailored itineraries, and dynamic interactions.

![Tokyo Image](assets/tokyo.png)

## Get Started
Build and run in moments.

```bash
cargo build
cargo run
```

Keep your repository fresh:
```bash
git pull
```

## Experience Tokyo
Dive into AI-generated itineraries, uncover cultural gems, and explore Tokyo with a seamless, interactive app. Powered by Rust and the Gemini-2.0-pro-exp-02-05 model, every interaction is rich, engaging, and tailored.

## Key Features
- **Enhanced Prompts**: Contextual prompts for precise, vibrant AI responses.
- **Tuned Creativity**: Temperature set to 0.7, Top K at 50, and Max Output Tokens at 150 for concise, creative outputs.
- **Secure Design**: Keep your `.env` file safe to protect API keys.

## Dependencies
- tokio
- colored
- rand
- dotenv
- reqwest
- serde
- serde_json

## Git Essentials
Take control of your workflow:
- Initialize: `git init`
- Stage: `git add .`
- Commit: `git commit -m "Your message"`
- Push: `git push -u origin branch-name`
- Fetch: `git fetch origin`
- Switch Branch: `git checkout branch-name`
- Create Branch: `git checkout -b new-branch-name`
- Clone: `git clone https://github.com/username/repo-name`
- Force Push: `git push --force`
- Amend Commit: `git commit --amend -m "Updated message"`
- Check Status: `git status`
- Pull: `git pull`

## AI Under the Hood
Tokyo Explorer + AI harnesses advanced probabilistic models:
- **Temperature**: Controls response randomness (0.7 for balanced creativity).
- **Top-K Sampling**: Selects from the top 50 probable tokens.
- **Top-P Sampling**: Targets tokens meeting a cumulative probability threshold.
- **Bayesian Inference**: Updates hypotheses with new data.
- **Markov Chain Monte Carlo (MCMC)**: Samples from complex distributions for dynamic outputs.

## Create a Release
Automate releases with GitHub Actions:
1. Commit and push changes.
2. Go to the "Actions" tab in your GitHub repo.
3. Select the "Generate Release" workflow.
4. Run the workflow and follow the prompts.

The `release.yml` file in `.github/workflows` handles building, releasing, and artifact uploads.

## Share Your Thoughts
We’d love to hear from you. Connect on [LinkedIn](https://www.linkedin.com/in/bniladridas) to share feedback or ideas.
