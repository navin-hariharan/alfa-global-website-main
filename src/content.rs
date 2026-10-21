//! Static content for the Group's enterprises. Wording follows the SEBI-safe
//! review: the Group invests only its own capital and none of these pages
//! offers advice, broking, portfolio management, research or returns.

pub struct Card {
    pub title: &'static str,
    pub desc: &'static str,
    pub icon: &'static str,
    pub items: &'static [&'static str],
}

/// One block on a company page. `kind` picks the layout:
/// "lead" (paragraphs), "list" (checklist), "cards" (grid) or "statement" (dark band).
pub struct Section {
    pub kind: &'static str,
    pub eyebrow: &'static str,
    pub title: &'static str,
    pub paras: &'static [&'static str],
    pub items: &'static [&'static str],
    pub cards: &'static [Card],
    pub outro: &'static [&'static str],
}

pub struct Company {
    pub slug: &'static str,
    pub name: &'static str,
    pub short: &'static str,
    pub tagline: &'static str,
    pub sector: &'static str,
    pub summary: &'static str,
    /// Path or URL of the company's own logo.
    pub logo: &'static str,
    pub website: &'static str,
    pub sections: &'static [Section],
}

const fn s(kind: &'static str, eyebrow: &'static str, title: &'static str) -> Section {
    Section { kind, eyebrow, title, paras: &[], items: &[], cards: &[], outro: &[] }
}

const fn c(title: &'static str, desc: &'static str, icon: &'static str) -> Card {
    Card { title, desc, icon, items: &[] }
}

pub static COMPANIES: &[Company] = &[
    Company {
        slug: "alfa-global-wealth-advisors",
        name: "Alfa Global Wealth Advisors",
        short: "Wealth Advisors",
        tagline: "Family Office & Treasury Platform",
        sector: "Family office",
        summary: "The Group’s family office platform, responsible for stewarding and allocating the Group’s own capital across its businesses and strategic holdings.",
        logo: "https://websitesbytechpioneers.com/alfaglobalnew/image/alfa-global-advisor-logo.png",
        website: "https://alfaglobalwealth.com/",
        sections: &[
            Section {
                paras: &[
                    "Alfa Global Wealth Advisors serves as the family office and treasury platform of Alfa Global Group. The Company stewards and allocates the Group’s own capital across its operating businesses and strategic holdings.",
                    "The Company does not have external clients. It does not provide investment advice, portfolio management or any other financial service to the public, and it does not accept funds from third parties.",
                ],
                ..s("lead", "Family Office & Treasury Platform", "Alfa Global Wealth Advisors")
            },
            Section {
                paras: &["The Company’s mandate is confined to the Group’s proprietary capital. It works with the Group’s leadership to set capital priorities, maintain appropriate liquidity, and support each operating business in pursuing its long-term objectives. All capital deployed is the Group’s own."],
                ..s("statement", "Mandate & Role", "Confined to the Group’s own capital")
            },
            Section {
                paras: &["The Company performs the following functions for Alfa Global Group and its entities. They cover the stewardship of the Group’s capital as well as the governance, risk and control frameworks that support it."],
                cards: &[
                    c("Stewardship of Group Capital", "Custody, preservation and long-term stewardship of the Group’s own capital in accordance with policies approved by the Board.", "lock"),
                    c("Capital Allocation Across Group Businesses", "Allocation of proprietary capital to the Group’s operating businesses and strategic holdings, aligned with the Group’s long-term priorities.", "layers"),
                    c("Treasury & Liquidity Management", "Management of the Group’s own liquidity, funding requirements and cash reserves to support the continuity of its operations.", "coins"),
                    c("Governance, Risk Management & Compliance", "Oversight of governance structures, risk frameworks, standard operating procedures and compliance processes across Group entities.", "scale"),
                    c("Internal Assurance", "Internal audits and reviews intended to uphold operational integrity and regulatory compliance within the Group.", "search"),
                    c("Internal Control over Financial Reporting", "Frameworks for financial reporting controls and oversight across the Group’s entities.", "doc"),
                ],
                ..s("cards", "Core Functions", "What the Company does for the Group")
            },
            Section {
                paras: &["The Company takes a measured, long-term approach to the Group’s capital. Before any capital is committed to a Group business or strategic holding, it is subject to internal due diligence, risk assessment and governance review. The Group’s capital is managed solely for the Group’s own account."],
                ..s("lead", "Approach", "Long-Term Capital Stewardship")
            },
            Section {
                paras: &["The Company is guided by clarity, transparency and long-term value. Alfa Global Wealth Advisors is a private limited company incorporated in Chennai in 2008, and it remains committed to regulatory compliance and operational integrity."],
                ..s("statement", "Values & Corporate Information", "Clarity, transparency and long-term value")
            },
        ],
    },
    Company {
        slug: "alfa-star-dynamics",
        name: "Alfa Star Dynamics",
        short: "Alfa Star Dynamics",
        tagline: "Strategic Holdings in Maritime and Industrial Enterprises",
        sector: "Strategic holdings",
        summary: "The Group’s strategic holdings arm, taking long-term stakes in maritime and industrial businesses alongside operating and industry partners.",
        logo: "/static/img/logos/alfa-star.png",
        website: "https://alfastardynamics.com/",
        sections: &[
            Section {
                paras: &["Alfa Star Dynamics is the strategic holdings arm of Alfa Global Group. The Company takes long-term stakes in maritime, infrastructure and industrial businesses alongside experienced operating partners. It invests only the Group’s own capital. It does not raise or manage funds for third parties and does not offer investment opportunities to the public."],
                ..s("lead", "Strategic Holdings Arm of Alfa Global Group", "About Alfa Star Dynamics")
            },
            Section {
                paras: &["The Company’s principal activity is acquiring and holding long-term stakes in established, revenue-generating businesses with sound fundamentals. It works alongside each business’s operating partners to strengthen governance, operations and enduring enterprise value."],
                cards: &[
                    Card {
                        title: "The Company considers holdings where",
                        desc: "",
                        icon: "search",
                        items: &[
                            "The business model is established and supported by visible cash flows",
                            "The business can scale with execution risk that is well understood and manageable",
                            "The business is supported by sector standing, sound governance or tangible asset backing",
                        ],
                    },
                    Card {
                        title: "The Group’s capital is committed through",
                        desc: "",
                        icon: "layers",
                        items: &["Direct equity participation", "Shareholder loans and other balance sheet support", "Joint ventures and partnerships with operating partners"],
                    },
                ],
                outro: &["Each commitment is made from the Group’s own resources, after internal due diligence and governance review, with a long-term ownership horizon."],
                ..s("cards", "Principal Activity", "Long-Term Strategic Holdings")
            },
            Section {
                paras: &["As part of managing its own balance sheet, the Company maintains a prudent treasury function covering its own resources only. This function covers:"],
                items: &[
                    "Management of the Company’s own liquidity reserves",
                    "Prudent placement of surplus funds for the Company’s own account",
                    "Management of risk on the Company’s own exposures",
                ],
                outro: &["Treasury activities are conducted solely for the Company’s own account to preserve liquidity and support its strategic holdings. The Company does not manage money, provide advice or undertake transactions for any third party."],
                ..s("list", "Own Account Only", "Treasury & Balance Sheet Management")
            },
            Section {
                paras: &["Through its holdings and operating partnerships, Alfa Star Dynamics is present in the maritime and marine logistics sector across the following activities:"],
                items: &[
                    "Acquisition, leasing and chartering of ships and marine assets",
                    "Freight forwarding and crew management",
                    "Marine infrastructure development and technical consulting",
                    "Coastal and international shipping operations",
                ],
                outro: &["These activities are carried out by the relevant operating businesses and their management teams, serving trade, logistics and infrastructure customers in the private and public sectors."],
                ..s("list", "Maritime", "Maritime & Marine Holdings")
            },
            Section {
                paras: &["The Company also holds direct interests and strategic partnerships in the following sectors:"],
                cards: &[
                    c("Infrastructure & Logistics", "", "building"),
                    c("Real Estate & Land Linked Assets", "", "pin"),
                    c("Emerging Technologies", "", "chip"),
                    c("Strategic Business Development", "", "chart"),
                ],
                outro: &["This multi-sector approach allows the Company to act both as a long-term owner and as a business enabler, supporting enduring enterprise value across the Group."],
                ..s("cards", "Beyond Maritime", "Broader Sectoral Interests")
            },
            Section {
                paras: &["Alfa Star Dynamics is a long-term owner and partner to the businesses in which it holds stakes. Drawing on strategic insight, operational expertise and the Group’s own capital, the Company aims to build durable enterprises of lasting value."],
                items: &[
                    "Long-term ownership of established, revenue-generating businesses",
                    "Disciplined selection and governance-led oversight",
                    "Close partnership with experienced operating management",
                    "Integrated capabilities across maritime, industrial and real-asset sectors",
                    "Investment of the Group’s own capital only, with no external investors",
                ],
                ..s("statement", "Why Alfa Star Dynamics", "A long-term owner and partner")
            },
        ],
    },
    Company {
        slug: "alfa-zillion-traders",
        name: "Alfa Zillion Traders",
        short: "Alfa Zillion Traders",
        tagline: "Technology-Led Proprietary Firm",
        sector: "Financial technology",
        summary: "A technology-led firm that builds in-house fintech platforms and quantitative tools, and trades solely on its own account.",
        logo: "/static/img/logos/alfa-zillion.png",
        website: "https://alfazillion.com/",
        sections: &[
            Section {
                paras: &[
                    "Alfa Zillion Traders LLP is a technology-led firm within Alfa Global Group. It develops in-house fintech platforms, data tools and quantitative models, and it trades solely on its own account.",
                    "The Firm is established under the Limited Liability Partnership Act, 2008. It brings together financial knowledge and technological capability for its own proprietary activities. It does not manage money, accept funds from any person, or provide investment or trading advice, recommendations or signals to others.",
                ],
                ..s("lead", "Technology-Led Proprietary Firm", "About Alfa Zillion Traders")
            },
            Section {
                paras: &[
                    "The Firm’s work rests on sound engineering, careful research and development for internal use, and prudent management of its own capital.",
                    "All trading activity is undertaken with the Firm’s own resources and is governed by internal risk limits, model validation and compliance oversight. The Firm does not hold mandates on behalf of others.",
                ],
                ..s("statement", "Operating Philosophy", "Engineering first, own capital only")
            },
            Section {
                cards: &[
                    c("In-House Fintech Platforms", "The Firm designs, builds and maintains proprietary technology platforms that support its own trading, risk management and operational processes.", "chip"),
                    c("Data & Analytics Tools", "The Firm develops internal data infrastructure and analytical tools for its own use, with an emphasis on data quality, reliability and sound engineering.", "bulb"),
                    c("Quantitative Models", "The Firm builds and tests quantitative models internally, under structured model governance and risk controls, for use in its own trading activities.", "cog"),
                    c("Proprietary Trading", "The Firm trades solely on its own account using its own capital. It does not manage money for others, accept funds from any person, or provide trading advice, recommendations or signals to third parties.", "chart"),
                ],
                ..s("cards", "Core Capabilities", "What the Firm builds")
            },
            Section {
                paras: &["The Firm’s activities are proprietary in nature and are conducted within the following boundaries:"],
                items: &[
                    "Trading conducted solely with the Firm’s own capital",
                    "No acceptance of funds, deposits or mandates from any person",
                    "No investment advice, research, recommendations or trading signals to third parties",
                    "Technology platforms and tools developed for internal use",
                    "Risk controls and compliance oversight applied across all activities",
                ],
                ..s("list", "Boundaries", "Scope of Activities")
            },
            Section {
                paras: &["Alfa Zillion Traders LLP is committed to building durable technology capabilities and to conducting its proprietary activities with discipline, integrity and full regard for applicable regulations."],
                items: &[
                    "Engineering Discipline in Platform Development",
                    "Rigorous Model Governance and Risk Controls",
                    "Integrity, Transparency and Regulatory Compliance",
                    "A Long-Term Commitment to Technology and Talent",
                ],
                ..s("statement", "Guiding Principles", "Discipline and integrity")
            },
        ],
    },
    Company {
        slug: "gazillion-capital",
        name: "Gazillion Capital",
        short: "Gazillion Capital",
        tagline: "A Financial Services Company of Alfa Global Group",
        sector: "Financial services",
        summary: "A separately operated financial services company with its own management and governance, and a focus on financial literacy.",
        logo: "/static/img/logos/gazillion.png",
        website: "https://www.gazcap.com/",
        sections: &[
            Section {
                paras: &[
                    "Gazillion Capital Private Limited is a separately operated company of Alfa Global Group in the financial services sector. It has its own management and governance structure, and it is guided by the Group’s commitment to integrity, transparency and responsible conduct.",
                    "The Company supports financial literacy initiatives intended to improve general awareness of how financial markets function. This page provides corporate information only. It does not offer broking, distribution of financial products, or investment advice to the public, and nothing on it constitutes a solicitation or recommendation. Any regulated activities of the Company are undertaken separately, under and in accordance with the applicable registrations.",
                ],
                ..s("lead", "Group Company in Financial Services", "About Gazillion Capital")
            },
            Section {
                cards: &[
                    Card {
                        title: "Financial Literacy Initiatives",
                        desc: "",
                        icon: "bulb",
                        items: &[
                            "Awareness programmes on the basic functioning of financial markets",
                            "Educational sessions on savings, risk and responsible financial conduct",
                            "Information on investor rights and regulatory safeguards",
                            "General educational content that does not constitute investment advice",
                        ],
                    },
                    Card {
                        title: "Corporate Principles",
                        desc: "",
                        icon: "scale",
                        items: &[
                            "Sound corporate governance",
                            "Compliance with applicable laws and regulations",
                            "Transparency in corporate communication",
                            "Ethical standards of conduct",
                            "Protection of data and confidentiality",
                        ],
                    },
                    Card {
                        title: "Governance & Compliance",
                        desc: "",
                        icon: "shield",
                        items: &[
                            "Board-level oversight of the Company’s affairs",
                            "Internal policies and compliance procedures",
                            "Risk management frameworks",
                            "Regulated activities undertaken only under applicable registrations",
                        ],
                    },
                    Card {
                        title: "Relationship with the Group",
                        desc: "",
                        icon: "handshake",
                        items: &[
                            "A separately operated company within Alfa Global Group",
                            "Its own management and governance structure",
                            "Alignment with the Group’s standards of integrity",
                            "A long-term orientation consistent with the Group’s values",
                        ],
                    },
                    Card {
                        title: "Financial Education Programmes",
                        desc: "",
                        icon: "users",
                        items: &[
                            "Introductory programmes on how capital markets function",
                            "Sessions on risk awareness and financial discipline",
                            "Workshops for students and working professionals",
                            "Content intended for general awareness only",
                        ],
                    },
                ],
                ..s("cards", "Gazillion Capital Initiatives", "Literacy, governance and conduct")
            },
            Section {
                paras: &[
                    "Gazillion Capital believes that sound financial understanding contributes to a more resilient economy and society. The Company’s philosophy rests on responsible conduct, strong governance and respect for the regulatory framework within which it operates.",
                    "The Company’s educational initiatives are intended for general awareness only and do not constitute investment advice, research or recommendations.",
                ],
                ..s("statement", "Philosophy", "Sound understanding, responsible conduct")
            },
            Section {
                cards: &[
                    c("Separate Operation", "Operated as a distinct company within Alfa Global Group", "layers"),
                    c("Experienced Leadership", "Managed by professionals with experience in financial services", "users"),
                    c("Governance First", "Board oversight, internal controls and compliance procedures", "scale"),
                    c("Financial Literacy", "A commitment to raising general awareness of financial matters", "bulb"),
                    c("Responsible Technology", "Prudent use of technology in the Company’s operations", "chip"),
                    c("Integrity", "Transparent, ethical and accountable conduct", "shield"),
                ],
                outro: &[
                    "Gazillion Capital is committed to the highest standards of governance, compliance and ethical conduct in all its activities, and to supporting financial awareness in the communities it serves.",
                    "As a member of Alfa Global Group, the Company shares the Group’s long-term orientation and its commitment to integrity, transparency and accountability.",
                ],
                ..s("cards", "Guiding Principles", "Commitment")
            },
        ],
    },
    Company {
        slug: "mk-dairy-farms",
        name: "MK Dairy Farms",
        short: "MK Dairy Farms",
        tagline: "Dairy, Agriculture & Natural Resources",
        sector: "Agriculture",
        summary: "The Group’s agriculture and natural resources enterprise, connecting quality produce with established markets while strengthening rural economies.",
        logo: "https://websitesbytechpioneers.com/alfaglobalnew/image/mk-logo-png.png",
        website: "https://mkdairyfarms.com/",
        sections: &[
            Section {
                paras: &[
                    "MK Dairy Farms, a company of Alfa Global Group, is engaged in the production, procurement, import, export and distribution of a wide range of dairy, agricultural and agro-based products. Its portfolio includes milk, cream, butter, ghee, cheese, eggs, fruits, vegetables and edible oils, supplied to both retail and institutional customers.",
                    "The Company is also engaged in the import, export and distribution of natural river sand, minerals, granites, marbles and related materials.",
                ],
                ..s("lead", "Dairy, Agriculture & Natural Resources", "MK Dairy Farms")
            },
            Section {
                cards: &[
                    c("Production & Procurement", "Production and procurement of dairy, agricultural and agro-based products to consistent quality standards.", "leaf"),
                    c("Import & Export", "Import and export of dairy products and natural resources in international markets.", "globe"),
                    c("Distribution", "A distribution network serving both retail and institutional customers.", "ship"),
                    c("Quality Focus", "A commitment to freshness, purity and nutritional value across the Company’s product range.", "star"),
                ],
                ..s("cards", "Operations", "From farm to market")
            },
            Section {
                cards: &[
                    Card { title: "Dairy Products", desc: "", icon: "leaf", items: &["High quality milk", "Cream", "Butter", "Ghee", "Cheese"] },
                    Card { title: "Agricultural Products", desc: "", icon: "spark", items: &["Eggs", "Fruits", "Vegetables", "Edible oils"] },
                    Card { title: "Natural Resources", desc: "", icon: "building", items: &["Natural river sand", "Minerals", "Granites", "Marbles", "Related materials"] },
                ],
                ..s("cards", "Product Portfolio", "What MK Dairy Farms supplies")
            },
            Section {
                paras: &["Guided by its focus on freshness, purity and nutritional value, MK Dairy Farms seeks to contribute to healthier lifestyles and to support the agricultural and natural resource economy."],
                ..s("statement", "Mission", "Freshness, purity and nutritional value")
            },
        ],
    },
    Company {
        slug: "cyberhakz",
        name: "Cyberhakz",
        short: "Cyberhakz",
        tagline: "Cybersecurity Services",
        sector: "Cyber security",
        summary: "The Group’s cyber security enterprise, providing enterprise-grade protection against evolving digital threats, from infrastructure security to regulatory compliance.",
        logo: "/static/img/logos/cyberhakz.png",
        website: "https://cyberhakz.com/",
        sections: &[
            Section {
                paras: &[
                    "Cyberhakz, a company of Alfa Global Group, provides cybersecurity services to organisations. Its team of experienced professionals specialises in penetration testing, threat detection, incident response and strategic security consulting, helping businesses protect themselves against evolving cyber threats.",
                    "The Company uses advanced tools, AI-based automation and contemporary methodologies to address emerging risks. Its work rests on transparency and ethical practice, and each engagement is shaped to the specific requirements of the organisation concerned.",
                ],
                ..s("lead", "Cybersecurity Services", "Cyberhakz")
            },
            Section {
                paras: &["The Company’s principal service lines for protecting organisations against evolving cyber threats."],
                cards: &[
                    c("Penetration Testing", "Structured security assessments designed to identify vulnerabilities before they can be exploited.", "search"),
                    c("Threat Detection", "Continuous monitoring and real-time threat detection supported by AI-based automation.", "eye"),
                    c("Incident Response", "Timely response and recovery services intended to contain and limit the impact of security incidents.", "bolt"),
                    c("Strategic Security Consulting", "Guidance on security strategy aligned with the specific requirements of each organisation.", "compass"),
                ],
                ..s("cards", "Services", "Protection across the lifecycle")
            },
            Section {
                cards: &[
                    c("Proactive Security", "A preventive approach to security that anticipates and addresses emerging threats.", "shield"),
                    c("Experienced Professionals", "A team of cybersecurity professionals with substantial industry knowledge.", "users"),
                    c("Innovation & Technology", "Use of advanced tools, AI-based automation and contemporary methodologies.", "chip"),
                    c("Transparency & Ethics", "A foundation of ethical practice, confidentiality and accountability to the organisations it serves.", "scale"),
                ],
                ..s("cards", "Distinguishing Strengths", "Why organisations choose Cyberhakz")
            },
            Section {
                paras: &["Cyberhakz is led by its founder, Navin Hariharan. The Company brings industry knowledge and a sustained commitment to excellence to complex cybersecurity challenges in a rapidly changing digital environment."],
                ..s("statement", "Leadership", "Led by its founder, Navin Hariharan")
            },
        ],
    },
    Company {
        slug: "dhanveir-groups-shipping",
        name: "Dhanveir Groups Shipping",
        short: "Dhanveir Shipping",
        tagline: "Strategic Maritime Platform | Indian-Flagged Vessel Operations",
        sector: "Maritime",
        summary: "The Group’s strategic maritime platform, established to build, own, and operate Indian-flagged vessels across key cargo segments, including dry bulk.",
        logo: "/static/img/logos/dhanveir.png",
        website: "https://www.dhanveirgroups.com/",
        sections: &[
            Section {
                paras: &[
                    "Dhanveir Groups Shipping Private Limited represents the Group's strategic maritime platform, established to build, own, and operate Indian-flagged vessels across key cargo segments. The company's current focus is dry bulk transportation, including sand, coal, and minerals, forming the foundation of a broader long-term vision to develop a diversified fleet spanning multiple vessel classes and trade routes.",
                    "The enterprise is led by industry professionals with over three decades of direct maritime experience across ship operations, technical management, regulatory compliance, and strategic fleet development. This depth of experience supports disciplined decision-making and execution in line with international operational standards.",
                ],
                ..s("lead", "Strategic Maritime Platform", "Dhanveir Groups Shipping Private Limited")
            },
            Section {
                paras: &["Dhanveir's technical backbone comprises Master Mariners, Marine Engineers, and international maritime advisors whose collective experience includes vessel acquisition, fleet operations, and cross-border maritime consulting. The leadership's exposure to global regulatory institutions and international shipping networks further strengthens the company's governance, compliance framework, and access to industry intelligence."],
                ..s("statement", "Technical Backbone & Leadership", "Master Mariners at the helm")
            },
            Section {
                paras: &["The Company’s principal operational areas and cargo segments."],
                items: &[
                    "Dry bulk transportation including sand, coal, and minerals",
                    "Building, owning, and operating Indian-flagged vessels",
                    "Fleet operations and technical management",
                    "Cross-border maritime consulting",
                    "Coastal and international trade corridors",
                ],
                ..s("list", "Operations", "Where Dhanveir operates")
            },
            Section {
                cards: &[
                    c("Maritime Expertise", "Over three decades of direct maritime experience across ship operations, technical management, and regulatory compliance.", "anchor"),
                    c("Technical Leadership", "Led by Master Mariners, Marine Engineers, and international maritime advisors with vessel acquisition and fleet operations expertise.", "compass"),
                    c("Regulatory Excellence", "Sound governance, a robust compliance framework and engagement with international maritime institutions.", "scale"),
                    c("Strategic Growth", "A structured expansion strategy focused on fleet scale, operational efficiency and participation in coastal and international trade.", "chart"),
                ],
                ..s("cards", "Key Strengths", "Built for the long voyage")
            },
            Section {
                paras: &["Positioned as a long-term asset platform within the Group, Dhanveir is pursuing a structured expansion strategy focused on fleet scale, operational efficiency, and participation in both coastal and international trade corridors. The company's objective is to build a durable maritime enterprise defined by disciplined capital deployment, technical excellence, and sustainable growth."],
                ..s("statement", "Vision", "A durable maritime enterprise")
            },
        ],
    },
];

pub fn company(slug: &str) -> Option<&'static Company> {
    COMPANIES.iter().find(|c| c.slug == slug)
}
