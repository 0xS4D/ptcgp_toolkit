import { routes } from '@/Routes';
import { Link } from 'react-router';

export default function Nav() {
  return (
    <nav className="text-xs">
      <ul className="flex h-full items-center gap-2">
        {routes[0].children.map((item, i) => (
          <Link to={item.path || '/'} className="text-zinc-500 hover:text-zinc-400" key={i}>
            {item.name}
          </Link>
        ))}
      </ul>
    </nav>
  );
}
