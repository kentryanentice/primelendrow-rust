import { Suspense, lazy } from 'react'
import { BrowserRouter, Routes, Route } from 'react-router-dom'

const SEOProvider = lazy(() => import('./SEOProvider'))

const Landing = lazy(() => import('../pages/Landing'))
const PrimeLendRow = lazy(() => import('../core/PrimeLendRow'))

type SEOConfigTypes = {
	title: string
	description: string
}

const seoConfigTypes: SEOConfigTypes = {
	title: 'Prime LendRow Official',
	description: 'PrimeLendRow: The global standard for decentralized, ethical P2P lending on ICP.',
}

function RoutesProvider() {
	 return (

		<>

			<BrowserRouter>

                <Suspense fallback={null}>

                    {/* <SessionProvider> */}

                        <Routes>
                            
                            <Route path='/' element={<> <SEOProvider {...seoConfigTypes} /> <Landing /> </>} />


                        </Routes>

                    {/* </SessionProvider> */}

                </Suspense>

			</BrowserRouter>

			<PrimeLendRow />

		</>

	)
}

export default RoutesProvider