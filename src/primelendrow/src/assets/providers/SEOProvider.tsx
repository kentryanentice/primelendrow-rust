import { Helmet } from 'react-helmet-async'

type SEOProviderTypes = {
    title?: string
    description?: string
    keywords?: string
    image?: string
    noindex?: boolean
}

function SEOProvider({
    title = 'PrimeLendRow – Decentralized P2P Lending on ICP',
    description = 'PrimeLendRow is the global standard for decentralized, ethical P2P lending on the Internet Computer blockchain.',
    keywords = 'PrimeLendRow, ICP, Internet Computer Protocol, Blockchain, Internet Identity, NFID, DeFi, P2P Lending, Virtual Wallet, Borrowers, Lenders',
    image = '/preview.webp',
    noindex = false,
}: SEOProviderTypes) {
    const canonical = typeof window !== 'undefined' ? window.location.href : ''

    return (
        <Helmet>
            <title>{title}</title>
            <meta name='description' content={description} />
            <meta name='keywords' content={keywords} />
            <meta name='author' content='PrimeLendRow' />
            <link rel='canonical' href={canonical} />
            {noindex && <meta name='robots' content='noindex,nofollow' />}
            <meta property='og:title' content={title} />
            <meta property='og:description' content={description} />
            <meta property='og:type' content='website' />
            <meta property='og:url' content={canonical} />
            <meta property='og:image' content={image} />
            <meta property='og:site_name' content='PrimeLendRow' />
            <meta name='twitter:card' content='summary_large_image' />
            <meta name='twitter:title' content={title} />
            <meta name='twitter:description' content={description} />
            <meta name='twitter:image' content={image} />
        </Helmet>
    )
}

export default SEOProvider