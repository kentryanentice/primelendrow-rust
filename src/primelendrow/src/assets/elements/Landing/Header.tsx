import { useState, useEffect } from 'react'

function Header() {
    type Theme = 'light' | 'dark'
    
    const [theme, setTheme] = useState<Theme>(() => {
        if (typeof window !== 'undefined') {
            const stored = localStorage.getItem('theme') as Theme | null
            if (stored === 'light' || stored === 'dark') return stored
            return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
        }
        return 'light'
    })

    useEffect(() => {
        document.documentElement.setAttribute('data-theme', theme)
        localStorage.setItem('theme', theme)
    }, [theme])

    const toggleTheme = () => setTheme(prev => (prev === 'light' ? 'dark' : 'light'))

    return (
        <>
        
            <header className='landing-header'>

                <img className='landing-logo' src='./pictures/logo.webp' alt='PrimeLendRow'/>

                <span className='landing-title'>Prime LendRow</span>


                <span className='landing-buttons'>

                    <span className='login'>LogIn</span>

                    <span className='register'>Register</span>

                    <i  className={theme === 'light' ? 'bx bx-sun' : 'bx bxs-sun'} onClick={toggleTheme}/>

                </span>

            </header>
        
        </>
    )
}

export default Header