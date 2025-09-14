// 開発用サンプルデータとモックデータの統合エクスポート

// 学術論文の詳細データ
export const mockPaperDetail = {
    id: 1,
    title: 'Attention Is All You Need',
    authors: [
        {
            name: 'Ashish Vaswani',
            h_index: 45,
            link: 'https://scholar.google.com/citations?user=example1'
        },
        {
            name: 'Noam Shazeer',
            h_index: 38,
            link: 'https://scholar.google.com/citations?user=example2'
        },
        {
            name: 'Niki Parmar',
            h_index: 25,
            link: 'https://scholar.google.com/citations?user=example3'
        },
        {
            name: 'Jakob Uszkoreit',
            h_index: 28,
            link: 'https://scholar.google.com/citations?user=example4'
        }
    ],
    abstract_text: 'The dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism. We propose a new simple network architecture, the Transformer, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely. Experiments on two machine translation tasks show these models to be superior in quality while being more parallelizable and requiring significantly less time to train.',
    url: 'https://arxiv.org/abs/1706.03762',
    published_date: '2017-06-12',
    journal: 'Advances in Neural Information Processing Systems',
    primary_category: 'cs.CL',
    citation_count: 78542,
    reference_count: 42,
    influential_citation_count: 12456,
    keywords: ['Attention Mechanism', 'Transformer', 'Neural Machine Translation', 'Deep Learning', 'Natural Language Processing'],
    background_and_purpose: 'Recurrent neural networks, long short-term memory and gated recurrent neural networks in particular, have been firmly established as state of the art approaches in sequence modeling and transduction problems such as language modeling and machine translation. Numerous efforts have since continued to push the boundaries of recurrent language models and encoder-decoder architectures.',
    methodology: 'The goal of reducing sequential computation also forms the foundation of the Extended Neural GPU, ByteNet and ConvS2S, all of which use convolutional neural networks as basic building block, computing hidden representations in parallel for all input and output positions. In these models, the number of operations required to relate signals from two arbitrary input or output positions grows in the distance between positions, linearly for ConvS2S and logarithmically for ByteNet.',
    dataset: 'We trained on the standard WMT 2014 English-German dataset consisting of about 4.5 million sentence pairs. We also used the larger WMT 2014 English-French dataset consisting of 36M sentences and split tokens into a 32000 word-piece vocabulary.',
    results: 'We evaluate our models on two machine translation tasks: WMT 2014 English-to-German and WMT 2014 English-to-French. For the smaller English-German dataset, we achieved a BLEU score of 28.4, which is competitive with the best previously reported results. For the larger English-French dataset, we achieved a BLEU score of 41.8, establishing a new state-of-the-art.',
    advantages_limitations_and_future_work: 'We plan to extend the Transformer to problems involving input and output modalities other than text, such as images, audio and video. Making generation less sequential is another research goals of ours. We also plan to investigate local, restricted attention mechanisms to efficiently handle very long sequences.',
    text: [
        '# Introduction\n\nThe dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism.\n\nWe propose a new simple network architecture, the Transformer, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely.',
        '# Related Work\n\nExtended Neural GPU, ByteNet and ConvS2S all use convolutional neural networks as basic building block, computing hidden representations in parallel for all input and output positions.\n\nIn these models, the number of operations required to relate signals from two arbitrary input or output positions grows in the distance between positions, linearly for ConvS2S and logarithmically for ByteNet.',
        '# Model Architecture\n\nThe Transformer follows this overall architecture using stacked self-attention and point-wise, fully connected layers for both the encoder and decoder, shown in the left and right halves of Figure 1, respectively.\n\n## Encoder and Decoder Stacks\n\n**Encoder:** The encoder is composed of a stack of N = 6 identical layers. Each layer has two sub-layers. The first is a multi-head self-attention mechanism, and the second is a simple, position-wise fully connected feed-forward network.',
        '# Attention\n\nAn attention function can be described as mapping a query and a set of key-value pairs to an output, where the query, keys, values, and output are all vectors.\n\n## Scaled Dot-Product Attention\n\nWe call our particular attention "Scaled Dot-Product Attention". The input consists of queries and keys of dimension dk, and values of dimension dv.'
    ],
    bibtex: `@inproceedings{vaswani2017attention,
  title={Attention is all you need},
  author={Vaswani, Ashish and Shazeer, Noam and Parmar, Niki and Uszkoreit, Jakob and Jones, Llion and Gomez, Aidan N and Kaiser, Lukasz and Polosukhin, Illia},
  booktitle={Advances in neural information processing systems},
  pages={5998--6008},
  year={2017}
}`
};

// 学術論文リストデータ
export const academicPapers = [
    {
        paper_id: 1,
        title: "Attention Is All You Need",
        authors: "Ashish Vaswani, Noam Shazeer, Niki Parmar, Jakob Uszkoreit, Llion Jones, Aidan N. Gomez, Lukasz Kaiser, Illia Polosukhin",
        published_date: "2017-06-12",
        updated_at: "2017-06-15T14:30:25",
        journal: "Advances in Neural Information Processing Systems",
        abstract: "The dominant sequence transduction models are based on complex recurrent or convolutional neural networks that include an encoder and a decoder. The best performing models also connect the encoder and decoder through an attention mechanism. We propose a new simple network architecture, the Transformer, based solely on attention mechanisms, dispensing with recurrence and convolutions entirely.",
        url: "https://arxiv.org/abs/1706.03762",
        keywords: ["Attention Mechanism", "Transformer", "Neural Machine Translation"],
        primary_category: "cs.CL"
    },
    {
        paper_id: 2,
        title: "BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding",
        authors: "Jacob Devlin, Ming-Wei Chang, Kenton Lee, Kristina Toutanova",
        published_date: "2018-10-11",
        updated_at: "2018-10-11T16:45:12",
        journal: "North American Chapter of the Association for Computational Linguistics",
        abstract: "We introduce a new language representation model called BERT, which stands for Bidirectional Encoder Representations from Transformers. Unlike recent language representation models, BERT is designed to pre-train deep bidirectional representations from unlabeled text by jointly conditioning on both left and right context in all layers.",
        url: "https://arxiv.org/abs/1810.04805",
        keywords: ["BERT", "Language Model", "Natural Language Processing"],
        primary_category: "cs.CL"
    },
    {
        paper_id: 3,
        title: "GPT-3: Language Models are Few-Shot Learners",
        authors: "Tom B. Brown, Benjamin Mann, Nick Ryder, Melanie Subbiah, Jared Kaplan, Prafulla Dhariwal, Arvind Neelakantan, Pranav Shyam, Girish Sastry, Amanda Askell, et al.",
        published_date: "2020-05-28",
        updated_at: "2020-05-28T09:15:33",
        journal: "Advances in Neural Information Processing Systems",
        abstract: "Recent work has demonstrated substantial gains on many NLP tasks and benchmarks by pre-training on a large corpus of text followed by fine-tuning on a specific task. While typically task-agnostic in architecture, this method still requires task-specific fine-tuning datasets of thousands or tens of thousands of examples.",
        url: "https://arxiv.org/abs/2005.14165",
        keywords: ["GPT-3", "Language Model", "Few-Shot Learning"],
        primary_category: "cs.CL"
    },
    {
        paper_id: 4,
        title: "ResNet: Deep Residual Learning for Image Recognition",
        authors: "Kaiming He, Xiangyu Zhang, Shaoqing Ren, Jian Sun",
        published_date: "2015-12-10",
        updated_at: "2015-12-10T11:22:44",
        journal: "IEEE Conference on Computer Vision and Pattern Recognition",
        abstract: "Deeper neural networks are more difficult to train. We present a residual learning framework to ease the training of networks that are substantially deeper than those used previously. We explicitly reformulate the layers as learning residual functions with reference to the layer inputs, instead of learning unreferenced functions.",
        url: "https://arxiv.org/abs/1512.03385",
        keywords: ["ResNet", "Deep Learning", "Computer Vision"],
        primary_category: "cs.CV"
    },
    {
        paper_id: 5,
        title: "Generative Adversarial Networks",
        authors: "Ian J. Goodfellow, Jean Pouget-Abadie, Mehdi Mirza, Bing Xu, David Warde-Farley, Sherjil Ozair, Aaron Courville, Yoshua Bengio",
        published_date: "2014-06-10",
        updated_at: "2014-06-10T13:05:17",
        journal: "Advances in Neural Information Processing Systems",
        abstract: "We propose a new framework for estimating generative models via an adversarial process, in which we simultaneously train two models: a generative model G that captures the data distribution, and a discriminative model D that estimates the probability that a sample came from the training data rather than G.",
        url: "https://arxiv.org/abs/1406.2661",
        keywords: ["GAN", "Generative Model", "Deep Learning"],
        primary_category: "cs.LG"
    }
];

// Webアーティクルデータ
export const webArticles = [
    {
        id: 1,
        timestamp: "2025-07-13",
        site_name: "TechCrunch",
        site_url: "https://techcrunch.com",
        title: "Latest AI Developments in 2025",
        summary: "A comprehensive overview of the latest artificial intelligence developments and their impact on various industries.",
        url: "https://techcrunch.com/ai-developments-2025",
        status: "new"
    },
    {
        id: 2,
        timestamp: "2025-07-12",
        site_name: "Wired",
        site_url: "https://wired.com",
        title: "Quantum Computing Breakthrough",
        summary: "Scientists achieve a major breakthrough in quantum computing that could revolutionize data processing.",
        url: "https://wired.com/quantum-computing-breakthrough",
        status: "archived"
    },
    {
        id: 3,
        timestamp: "2025-07-11",
        site_name: "The Verge",
        site_url: "https://theverge.com",
        title: "Meta's New VR Headset",
        summary: "Meta announces their latest VR headset with improved resolution and tracking capabilities.",
        url: "https://theverge.com/meta-new-vr-headset",
        status: "new"
    },
    {
        id: 4,
        timestamp: "2025-07-10",
        site_name: "Ars Technica",
        site_url: "https://arstechnica.com",
        title: "SpaceX Mars Mission Update",
        summary: "Latest updates on SpaceX's Mars colonization mission and technological developments.",
        url: "https://arstechnica.com/spacex-mars-mission-update",
        status: "new"
    },
    {
        id: 5,
        timestamp: "2025-07-09",
        site_name: "MIT Technology Review",
        site_url: "https://technologyreview.com",
        title: "Gene Therapy Advances",
        summary: "New gene therapy treatments show promising results in clinical trials for genetic disorders.",
        url: "https://technologyreview.com/gene-therapy-advances",
        status: "archived"
    }
];

// 統合されたサンプルデータオブジェクト
export const sampleData = {
    academicPapers,
    webArticles,
    mockPaperDetail
};

// デフォルトエクスポート
export default sampleData;
