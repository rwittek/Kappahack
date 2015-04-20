#include <windows.h>
#include <math.h>

#define CHECK_VALID( _v ) 0
#define Assert( _exp ) ((void)0)
class Vector					
{
public:
	float x, y, z;
	Vector(void); 
	Vector(float X, float Y, float Z);
	void Init(float ix=0.0f, float iy=0.0f, float iz=0.0f);
	bool IsValid() const;
	float operator[](int i) const;
	float& operator[](int i);
	inline void Zero();
	bool operator==(const Vector& v) const;
	bool operator!=(const Vector& v) const;	
	inline Vector&	operator+=(const Vector &v);			
	inline Vector&	operator-=(const Vector &v);		
	inline Vector&	operator*=(const Vector &v);			
	inline Vector&	operator*=(float s);
	inline Vector&	operator/=(const Vector &v);		
	inline Vector&	operator/=(float s);	
	inline Vector&	operator+=(float fl);
	inline Vector&	operator-=(float fl);
	inline float	Length() const;
	inline float LengthSqr(void) const
	{ 
		CHECK_VALID(*this);
		return (x*x + y*y + z*z);		
	}
	bool IsZero( float tolerance = 0.01f ) const
	{
		return (x > -tolerance && x < tolerance &&
				y > -tolerance && y < tolerance &&
				z > -tolerance && z < tolerance);
	}
	float	NormalizeInPlace();
	inline float	DistToSqr(const Vector &vOther) const;
	float	Dot(const Vector& vOther) const;			
	float	Length2D(void) const;		
	float	Length2DSqr(void) const;
	Vector& operator=(const Vector &vOther);
	Vector	operator-(void) const;
	Vector	operator+(const Vector& v) const;	
	Vector	operator-(const Vector& v) const;	
	Vector	operator*(const Vector& v) const;	
	Vector	operator/(const Vector& v) const;	
	Vector	operator*(float fl) const;
	Vector	operator/(float fl) const;
};
//===============================================
inline void Vector::Init( float ix, float iy, float iz )    
{ 
	x = ix; y = iy; z = iz;
	CHECK_VALID(*this);
}
//===============================================
inline Vector::Vector(float X, float Y, float Z)
{ 
	x = X; y = Y; z = Z;
	CHECK_VALID(*this);
}
//===============================================
inline Vector::Vector(void){ }
//===============================================
inline void Vector::Zero()
{
	x = y = z = 0.0f;
}
//===============================================
inline void VectorClear( Vector& a )
{
	a.x = a.y = a.z = 0.0f;
}
//===============================================
inline Vector& Vector::operator=(const Vector &vOther)	
{
	CHECK_VALID(vOther);
	x=vOther.x; y=vOther.y; z=vOther.z; 
	return *this; 
}
//===============================================
inline float& Vector::operator[](int i)
{
	Assert( (i >= 0) && (i < 3) );
	return ((float*)this)[i];
}
//===============================================
inline float Vector::operator[](int i) const
{
	Assert( (i >= 0) && (i < 3) );
	return ((float*)this)[i];
}
//===============================================
inline bool Vector::operator==( const Vector& src ) const
{
	CHECK_VALID(src);
	CHECK_VALID(*this);
	return (src.x == x) && (src.y == y) && (src.z == z);
}
//===============================================
inline bool Vector::operator!=( const Vector& src ) const
{
	CHECK_VALID(src);
	CHECK_VALID(*this);
	return (src.x != x) || (src.y != y) || (src.z != z);
}
//===============================================
inline void VectorCopy( const Vector& src, Vector& dst )
{
	CHECK_VALID(src);
	dst.x = src.x;
	dst.y = src.y;
	dst.z = src.z;
}
//===============================================
inline  Vector& Vector::operator+=(const Vector& v)	
{ 
	CHECK_VALID(*this);
	CHECK_VALID(v);
	x+=v.x; y+=v.y; z += v.z;	
	return *this;
}
//===============================================
inline  Vector& Vector::operator-=(const Vector& v)	
{ 
	CHECK_VALID(*this);
	CHECK_VALID(v);
	x-=v.x; y-=v.y; z -= v.z;	
	return *this;
}
//===============================================
inline  Vector& Vector::operator*=(float fl)	
{
	x *= fl;
	y *= fl;
	z *= fl;
	CHECK_VALID(*this);
	return *this;
}
//===============================================
inline  Vector& Vector::operator*=(const Vector& v)	
{ 
	CHECK_VALID(v);
	x *= v.x;
	y *= v.y;
	z *= v.z;
	CHECK_VALID(*this);
	return *this;
}
//===============================================
inline Vector&	Vector::operator+=(float fl) 
{
	x += fl;
	y += fl;
	z += fl;
	CHECK_VALID(*this);
	return *this;
}
//===============================================
inline Vector&	Vector::operator-=(float fl) 
{
	x -= fl;
	y -= fl;
	z -= fl;
	CHECK_VALID(*this);
	return *this;
}
//===============================================
inline  Vector& Vector::operator/=(float fl)	
{
	Assert( fl != 0.0f );
	float oofl = 1.0f / fl;
	x *= oofl;
	y *= oofl;
	z *= oofl;
	CHECK_VALID(*this);
	return *this;
}
//===============================================
inline  Vector& Vector::operator/=(const Vector& v)	
{ 
	CHECK_VALID(v);
	Assert( v.x != 0.0f && v.y != 0.0f && v.z != 0.0f );
	x /= v.x;
	y /= v.y;
	z /= v.z;
	CHECK_VALID(*this);
	return *this;
}
//===============================================
inline float Vector::Length(void) const	
{
	CHECK_VALID(*this);
	
	float root = 0.0f;

	float sqsr = x*x + y*y + z*z;

	asm
	("sqrtss xmm0, sqsr"
	 "movss $root, xmm0"
	);

	return root;
}
//===============================================
inline float Vector::Length2D(void) const
{
	CHECK_VALID(*this);
	
	float root = 0.0f;

	float sqst = x*x + y*y;

	asm
	("sqrtss xmm0, sqsr"
	 "movss $root, xmm0"
	);

	return root;
}
//===============================================
inline float Vector::Length2DSqr(void) const
{ 
	return (x*x + y*y); 
}
//===============================================
inline Vector CrossProduct(const Vector& a, const Vector& b) 
{ 
	return Vector( a.y*b.z - a.z*b.y, a.z*b.x - a.x*b.z, a.x*b.y - a.y*b.x ); 
}
//===============================================
float Vector::DistToSqr(const Vector &vOther) const
{
	Vector delta;

	delta.x = x - vOther.x;
	delta.y = y - vOther.y;
	delta.z = z - vOther.z;

	return delta.LengthSqr();
}
//===============================================
inline float Vector::NormalizeInPlace()
{
	Vector& v = *this;

	float iradius = 1.f / ( this->Length() + 1.192092896e-07F ); //FLT_EPSILON
	
	v.x *= iradius;
	v.y *= iradius;
	v.z *= iradius;
}
//===============================================
inline Vector Vector::operator+(const Vector& v) const	
{ 
	Vector res;
	res.x = x + v.x;
	res.y = y + v.y;
	res.z = z + v.z;
	return res;	
}
//===============================================
inline Vector Vector::operator-(const Vector& v) const	
{ 
	Vector res;
	res.x = x - v.x;
	res.y = y - v.y;
	res.z = z - v.z;
	return res;	
}
//===============================================
inline Vector Vector::operator*(float fl) const	
{ 
	Vector res;
	res.x = x * fl;
	res.y = y * fl;
	res.z = z * fl;
	return res;	
}
//===============================================
inline Vector Vector::operator*(const Vector& v) const	
{ 
	Vector res;
	res.x = x * v.x;
	res.y = y * v.y;
	res.z = z * v.z;
	return res;	
}
//===============================================
inline Vector Vector::operator/(float fl) const	
{ 
	Vector res;
	res.x = x / fl;
	res.y = y / fl;
	res.z = z / fl;
	return res;	
}
//===============================================
inline Vector Vector::operator/(const Vector& v) const	
{ 
	Vector res;
	res.x = x / v.x;
	res.y = y / v.y;
	res.z = z / v.z;
	return res;
}
inline float Vector::Dot( const Vector& vOther ) const
{
	const Vector& a = *this;
	
	return( a.x*vOther.x + a.y*vOther.y + a.z*vOther.z ); 
}

//Credits: Casual_Hacker
inline void**& getvtable( void* inst, size_t offset = 0 )
{
	return *reinterpret_cast<void***>( (size_t)inst + offset );
}
inline const void** getvtable( const void* inst, size_t offset = 0 )
{
	return *reinterpret_cast<const void***>( (size_t)inst + offset );
}
template< typename Fn >
inline Fn getvfunc( const void* inst, size_t index, size_t offset = 0 )
{
	return reinterpret_cast<Fn>( getvtable( inst, offset )[ index ] );
}

//#include "WeaponList.h"
//#include "CGlobalVars.h"
//#include "VMTHooks.h"

using namespace std;

#define WIN32_LEAN_AND_MEAN

typedef float matrix3x4[3][4];

typedef struct player_info_s
{
	char			name[32];
	int				userID;
	char			guid[33];
	unsigned long	friendsID;
	char			friendsName[32];
	bool			fakeplayer;
	bool			ishltv;
	unsigned long	customFiles[4];
	unsigned char	filesDownloaded;
} player_info_t;

class ClientClass
{
private:
	BYTE _chPadding[8];
public:
	char* chName;
	void* RecvTable;
	ClientClass* pNextClass;
	int iClassID;
};


enum TraceType_t
{
	TRACE_EVERYTHING = 0,
	TRACE_WORLD_ONLY,				// NOTE: This does *not* test static props!!!
	TRACE_ENTITIES_ONLY,			// NOTE: This version will *not* test static props
	TRACE_EVERYTHING_FILTER_PROPS,	// NOTE: This version will pass the IHandleEntity for props through the filter, unlike all other filters
};

class IHandleEntity {};
	extern "C" int & IHandleEntity_GetRefEHandle( IHandleEntity *_this )
	{
		typedef int & ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>(_this, 2)(_this);
	}

struct ITraceFilterVtable {
	bool (__thiscall *ShouldHitEntity)(PVOID *_this, IHandleEntity *pEntity, int contentsMask);
	TraceType_t (__thiscall *GetTraceType)(PVOID *_this);
};

class ITraceFilter {};

class CTraceFilterSkipEntity: public ITraceFilter
{
	public: 
	ITraceFilterVtable *vt;
	int skiphandle;
	CTraceFilterSkipEntity();
};

bool __thiscall CTraceFilterSkipEntity_ShouldHitEntity(PVOID *_this, IHandleEntity *pEntity, int contentsMask) {
	return IHandleEntity_GetRefEHandle(pEntity) != (((CTraceFilterSkipEntity *)_this) ->skiphandle);
}

TraceType_t __thiscall CTraceFilterSkipEntity_GetTraceType(PVOID *_this) {
	return TRACE_EVERYTHING;
}

ITraceFilterVtable CTraceFilterSkipEntityVtable;

CTraceFilterSkipEntity::CTraceFilterSkipEntity() {
	vt = &CTraceFilterSkipEntityVtable;
	vt->ShouldHitEntity = CTraceFilterSkipEntity_ShouldHitEntity;
	vt->GetTraceType = CTraceFilterSkipEntity_GetTraceType;
}

CTraceFilterSkipEntity global_tracefilter;
ITraceFilter *GLOBAL_TRACEFILTER_PTR = &global_tracefilter;
extern "C" void CTraceFilterSkipEntity_SetHandle(CTraceFilterSkipEntity *_this, int handle) {
	_this->skiphandle = handle;
}

class VectorAligned : public Vector {} __attribute__((aligned(16)));

struct Ray_t
{
	VectorAligned  m_Start;	// starting point, centered within the extents
	VectorAligned  m_Delta;	// direction + length of the ray
	VectorAligned  m_StartOffset;	// Add this to m_Start to get the actual ray start
	VectorAligned  m_Extents;	// Describes an axis aligned box extruded along a ray
	bool	m_IsRay;	// are the extents zero?
	bool	m_IsSwept;	// is delta != 0?
};


struct cplane_t
{
	Vector	normal;
	float	dist;
	byte	type;			// for fast side tests
	byte	signbits;		// signx + (signy<<1) + (signz<<1)
	byte	pad[2];
};


struct csurface_t
{
	const char	*name;
	short		surfaceProps;
	unsigned short	flags;		// BUGBUG: These are declared per surface, not per material, but this database is per-material now
};
class CBaseTrace
{
public:

	// these members are aligned!!
	Vector			startpos;				// start position
	Vector			endpos;					// final position
	cplane_t		plane;					// surface normal at impact

	float			fraction;				// time completed, 1.0 = didn't hit anything

	int				contents;				// contents on other side of surface hit
	unsigned short	dispFlags;				// displacement flags for marking surfaces with data

	bool			allsolid;				// if true, plane is not valid
	bool			startsolid;				// if true, the initial point was in a solid area
};
class trace_t : public CBaseTrace
{
public:

	float		fractionleftsolid;		// time we left a solid, only valid if we started in solid
	csurface_t	surface;				// surface hit (impact surface)

	int			hitgroup;				// 0 == generic, non-zero is specific body part
	short		physicsbone;			// physics bone hit by trace in studio

	void *m_pEnt; // FIXME: C_BaseEntity *

	// NOTE: this member is overloaded.
	// If hEnt points at the world entity, then this is the static prop index.
	// Otherwise, this is the hitbox index.
	int			hitbox;					// box hit by trace in studio
};

class CHLClient
{
public:
	ClientClass* GetAllClasses( void )
	{
		typedef ClientClass* ( __thiscall* OriginalFn )( PVOID ); //Anything inside a VTable is a __thiscall unless it completly disregards the thisptr. You can also call them as __stdcalls, but you won't have access to the __thisptr.
		return getvfunc<OriginalFn>( this, 8 )( this ); //Return the pointer to the head CClientClass.
	}
};

class CGlobals
{
public:
	float realtime;
	int framecount;
	float absoluteframetime;
	float curtime;
	float frametime;
	int maxclients;
	int tickcount;
	float interval_per_tick;
	float interpolation_amount;
};

class CUserCmd
{
public:
	virtual ~CUserCmd() {}; //Destructor 0
	int command_number; //4
	int tick_count; //8
	Vector viewangles; //C
	float forwardmove; //18
	float sidemove; //1C
	float upmove; //20
	int	buttons; //24
	byte impulse; //28
	int weaponselect; //2C
	int weaponsubtype; //30
	int random_seed; //34
	short mousedx; //38
	short mousedy; //3A
	bool hasbeenpredicted; //3C;
};
class CBaseEntity {};

typedef void (__thiscall* EstimateAbsVelocityFn)( CBaseEntity* thisptr, Vector& vel );

EstimateAbsVelocityFn ESTIMATE_ABS_VELOCITY = NULL;

	extern "C" Vector& CBaseEntity_GetAbsOrigin( CBaseEntity *_this )
	{
		typedef Vector& ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>(_this, 9)(_this);
	}
	extern "C" int & CBaseEntity_GetRefEHandle( CBaseEntity *_this )
	{
		typedef int & ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>(_this, 2)(_this);
	}
	extern "C" Vector& CBaseEntity_GetAbsAngles( CBaseEntity *_this )
	{
		typedef Vector& ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>(_this, 10)(_this);
	}
	extern "C" void CBaseEntity_UpdateGlowEffect( CBaseEntity *_this )
	{
		typedef void ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>(_this, 0x384 / 4)(_this);
	}
	extern "C" void CBaseEntity_EstimateAbsVelocity( CBaseEntity *_this, Vector &vel )
	{
        ESTIMATE_ABS_VELOCITY(_this, vel);
	}
/*
	void GetWorldSpaceCenter( Vector& vWorldSpaceCenter)
	{
		Vector vMin, vMax;
		this->GetRenderBounds( vMin, vMax );
		vWorldSpaceCenter = this->GetAbsOrigin();
		vWorldSpaceCenter.z += (vMin.z + vMax.z) / 2;
	}
	DWORD* GetModel( )
	{
		PVOID pRenderable = (PVOID)(this + 0x4);
		typedef DWORD* ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( pRenderable, 9 )( pRenderable );
	}
	bool SetupBones( matrix3x4 *pBoneToWorldOut, int nMaxBones, int boneMask, float currentTime )
	{
		PVOID pRenderable = (PVOID)(this + 0x4);
		typedef bool ( __thiscall* OriginalFn )( PVOID, matrix3x4*, int, int, float );
		return getvfunc<OriginalFn>( pRenderable, 16 )( pRenderable, pBoneToWorldOut, nMaxBones, boneMask, currentTime );
	}
    */ 
	extern "C" ClientClass* CBaseEntity_GetClientClass( CBaseEntity *_this )
	{
		PVOID pNetworkable = (PVOID)(_this + 0x8);
		typedef ClientClass* ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( pNetworkable, 2 )( pNetworkable );
	}
	extern "C" bool CBaseEntity_IsDormant( CBaseEntity *_this )
	{
		PVOID pNetworkable = (PVOID)(_this + 0x8);
		typedef bool ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( pNetworkable, 8 )( pNetworkable );
	}
	extern "C" int CBaseEntity_GetIndex( CBaseEntity *_this )
	{
		PVOID pNetworkable = (PVOID)(_this + 0x8);
		typedef int ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( pNetworkable, 9 )( pNetworkable );
	}
	
	extern "C" void CBaseEntity_GetRenderBounds( CBaseEntity *_this, Vector& mins, Vector& maxs )
	{
		PVOID pRenderable = (PVOID)(_this + 0x4);
		typedef void ( __thiscall* OriginalFn )( PVOID, Vector& , Vector& );
		getvfunc<OriginalFn>( pRenderable, 20)( pRenderable, mins, maxs );
	}

class EngineClient{};

	extern "C" void EngineClient_GetScreenSize( EngineClient *_this, int& width, int& height )
	{
		typedef void ( __thiscall* OriginalFn )( PVOID, int& , int& );
		return getvfunc<OriginalFn>( _this, 5 )( _this, width, height );
	}

	extern "C" bool EngineClient_GetPlayerInfo( EngineClient *_this, int ent_num, player_info_t *pinfo )
	{
		typedef bool ( __thiscall* OriginalFn )( PVOID, int, player_info_t * );
		return getvfunc<OriginalFn>(_this, 8)(_this, ent_num, pinfo );
	}
	extern "C" bool EngineClient_Con_IsVisible(EngineClient *_this )
	{
		typedef bool ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 11 )( _this );
	}
	extern "C" int EngineClient_GetLocalPlayer( EngineClient *_this )
	{
		typedef int ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 12 )( _this );
	}
	extern "C" float EngineClient_Time( EngineClient *_this )
	{
		typedef float ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 14 )( _this );
	}
	extern "C" void EngineClient_GetViewAngles( EngineClient *_this, Vector& va )
	{
		typedef void ( __thiscall* OriginalFn )( PVOID, Vector& va );
		return getvfunc<OriginalFn>( _this, 19 )( _this, va );
	}
	extern "C" void EngineClient_SetViewAngles( EngineClient *_this, Vector& va )
	{
		typedef void ( __thiscall* OriginalFn )( PVOID, Vector& va );
		return getvfunc<OriginalFn>( _this, 20 )( _this, va );
	}
	extern "C" int EngineClient_GetMaxClients( EngineClient *_this )
	{
		typedef int ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 21 )( _this );
	}
	extern "C" bool EngineClient_IsInGame( EngineClient *_this )
	{
		typedef bool ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 26 )( _this );
	}
	extern "C" bool EngineClient_IsConnected( EngineClient *_this )
	{
		typedef bool ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 27 )( _this );
	}
	extern "C" bool EngineClient_IsDrawingLoadingImage( EngineClient *_this  )
	{
		typedef bool ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 28 )( _this );
	}
	extern "C" const matrix3x4& EngineClient_WorldToScreenMatrix( EngineClient *_this )
	{
		typedef const matrix3x4& ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>(_this, 36)(_this);
	}
	extern "C" bool EngineClient_IsTakingScreenshot( EngineClient *_this  )
	{
		typedef bool ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 85 )( _this );
	}
	extern "C" DWORD* EngineClient_GetNetChannelInfo( EngineClient *_this  )
	{
		typedef DWORD* ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 72 )( _this );
	}
	extern "C" void EngineClient_ClientCmd_Unrestricted( EngineClient *_this, const char* chCommandString )
	{
		typedef void ( __thiscall* OriginalFn )( PVOID, const char * );
		return getvfunc<OriginalFn>( _this, 106 )( _this, chCommandString );
	}

class DebugOverlay;
	extern "C" void DebugOverlay_AddLineOverlay( EngineClient *_this, const Vector& origin, const Vector& dest, int r, int g, int b, bool noDepthTest, float duration)
	{
		typedef void ( __thiscall* OriginalFn )( PVOID, const Vector&, const Vector&, int, int, int, bool, float);
		return getvfunc<OriginalFn>( _this, 3)( _this, origin, dest, r, g, b, noDepthTest, duration);
	}

class IPanel
{
public:
	const char *GetName(unsigned int vguiPanel)
	{
		typedef const char* ( __thiscall* OriginalFn )( PVOID, unsigned int );
		return getvfunc<OriginalFn>( this, 36 )( this, vguiPanel );
	}
};

class ISurface
{
public:
	void DrawSetColor(int r, int g, int b, int a)
	{
		typedef void(__thiscall* OriginalFn)(PVOID, int, int, int, int);
		getvfunc<OriginalFn>(this, 11)(this, r, g, b, a);
	}
	void DrawFilledRect(int x0, int y0, int x1, int y1)
	{
		typedef void(__thiscall* OriginalFn)(PVOID, int, int, int, int);
		getvfunc<OriginalFn>(this, 12)(this, x0, y0, x1, y1);
	}
	void DrawOutlinedRect(int x0, int y0, int x1, int y1)
	{
		typedef void(__thiscall* OriginalFn)(PVOID, int, int, int, int);
		getvfunc<OriginalFn>(this, 14)(this, x0, y0, x1, y1);
	}
	void DrawSetTextFont(unsigned long font)
	{
		typedef void(__thiscall* OriginalFn)(PVOID, unsigned long);
		getvfunc<OriginalFn>(this, 17)(this, font);
	}
	void DrawSetTextColor(int r, int g, int b, int a)
	{
		typedef void(__thiscall* OriginalFn)(PVOID, int, int, int, int);
		getvfunc<OriginalFn>(this, 19)(this, r, g, b, a);
	}
	void DrawSetTextPos(int x, int y)
	{
		typedef void(__thiscall* OriginalFn)(PVOID, int, int);
		getvfunc<OriginalFn>(this, 20)(this, x, y);
	}
	void DrawPrintText(const wchar_t *text, int textLen)
	{
		typedef void(__thiscall* OriginalFn)(PVOID, const wchar_t *, int, int);
		return getvfunc<OriginalFn>(this, 22)(this, text, textLen, 0);
	}
	unsigned long CreateFont()
	{
		typedef unsigned int(__thiscall* OriginalFn)(PVOID);
		return getvfunc<OriginalFn>(this, 66)(this);
	}
	void SetFontGlyphSet(unsigned long &font, const char *windowsFontName, int tall, int weight, int blur, int scanlines, int flags)
	{
		typedef void(__thiscall* OriginalFn)(PVOID, unsigned long, const char*, int, int, int, int, int, int, int);
		getvfunc<OriginalFn>(this, 67)(this, font, windowsFontName, tall, weight, blur, scanlines, flags, 0, 0);
	}
	void GetTextSize(unsigned long font, const wchar_t *text, int &wide, int &tall)
	{
		typedef void(__thiscall* OriginalFn)(PVOID, unsigned long, const wchar_t *, int&, int&);
		getvfunc<OriginalFn>(this, 75)(this, font, text, wide, tall);
	}
};

class CEntList;
	extern "C" CBaseEntity* CEntList_GetClientEntity( CEntList *_this, int entnum )
	{
		typedef CBaseEntity* ( __thiscall* OriginalFn )( PVOID, int );
		return getvfunc<OriginalFn>( _this, 3 )( _this, entnum );
	}
	extern "C" CBaseEntity* CEntList_GetClientEntityFromHandle( CEntList *_this, int hEnt )
	{
		typedef CBaseEntity* ( __thiscall* OriginalFn )( PVOID, int );
		return getvfunc<OriginalFn>( _this, 4 )( _this, hEnt );
	}
	extern "C" int CEntList_GetHighestEntityIndex(CEntList *_this)
	{
		typedef int ( __thiscall* OriginalFn )( PVOID );
		return getvfunc<OriginalFn>( _this, 6 )( _this );
	}

class CEngineTrace;
	extern "C" void CEngineTrace_TraceRay( CEngineTrace *_this, const Ray_t &ray, unsigned int fMask, ITraceFilter *pTraceFilter, trace_t *pTrace)
	{
		typedef void ( __thiscall* OriginalFn )( PVOID, const Ray_t &, unsigned int, ITraceFilter *, trace_t * );
		getvfunc<OriginalFn>( _this, 4 )( _this, ray, fMask, pTraceFilter, pTrace );
	}
class INetChannelInfo;
	extern "C" float INetChannelInfo_GetLatency( INetChannelInfo *_this, int flow) {
		typedef float ( __thiscall* OriginalFn )( PVOID, int flow); 
		return getvfunc<OriginalFn>( _this, 9 )( _this, flow); 
	}
enum playercontrols
{
	IN_ATTACK = (1 << 0),
	IN_JUMP	= (1 << 1),
	IN_DUCK = (1 << 2),
	IN_FORWARD = (1 << 3),
	IN_BACK = (1 << 4),
	IN_USE = (1 << 5),
	IN_CANCEL = (1 << 6),
	IN_LEFT = (1 << 7),
	IN_RIGHT = (1 << 8),
	IN_MOVELEFT = (1 << 9),
	IN_MOVERIGHT = (1 << 10),
	IN_ATTACK2 = (1 << 11),
	IN_RUN = (1 << 12),
	IN_RELOAD = (1 << 13),
	IN_ALT1 = (1 << 14),
	IN_ALT2 = (1 << 15),
	IN_SCORE = (1 << 16),	// Used by client.dll for when scoreboard is held down
	IN_SPEED = (1 << 17),	// Player is holding the speed key
	IN_WALK = (1 << 18),	// Player holding walk key
	IN_ZOOM	= (1 << 19),	// Zoom key for HUD zoom
	IN_WEAPON1 = (1 << 20),	// weapon defines these bits
	IN_WEAPON2 = (1 << 21),	// weapon defines these bits
	IN_BULLRUSH = (1 << 22),
};

enum tf_cond 
{ 
    TFCond_Slowed = (1 << 0), //Toggled when a player is slowed down. 
    TFCond_Zoomed = (1 << 1), //Toggled when a player is zoomed. 
    TFCond_Disguising = (1 << 2), //Toggled when a Spy is disguising.  
    TFCond_Disguised = (1 << 3), //Toggled when a Spy is disguised. 
    TFCond_Cloaked = (1 << 4), //Toggled when a Spy is invisible. 
    TFCond_Ubercharged = (1 << 5), //Toggled when a player is ?berCharged. 
    TFCond_TeleportedGlow = (1 << 6), //Toggled when someone leaves a teleporter and has glow beneath their feet. 
    TFCond_Taunting = (1 << 7), //Toggled when a player is taunting. 
    TFCond_UberchargeFading = (1 << 8), //Toggled when the ?berCharge is fading. 
    TFCond_CloakFlicker = (1 << 9), //Toggled when a Spy is visible during cloak. 
    TFCond_Teleporting = (1 << 10), //Only activates for a brief second when the player is being teleported; not very useful. 
    TFCond_Kritzkrieged = (1 << 11), //Toggled when a player is being crit buffed by the KritzKrieg. 
    TFCond_TmpDamageBonus = (1 << 12), //Unknown what this is for. Name taken from the AlliedModders SDK. 
    TFCond_DeadRingered = (1 << 13), //Toggled when a player is taking reduced damage from the Deadringer. 
    TFCond_Bonked = (1 << 14), //Toggled when a player is under the effects of The Bonk! Atomic Punch. 
    TFCond_Stunned = (1 << 15), //Toggled when a player's speed is reduced from airblast or a Sandman ball. 
    TFCond_Buffed = (1 << 16), //Toggled when a player is within range of an activated Buff Banner. 
    TFCond_Charging = (1 << 17), //Toggled when a Demoman charges with the shield. 
    TFCond_DemoBuff = (1 << 18), //Toggled when a Demoman has heads from the Eyelander. 
    TFCond_CritCola = (1 << 19), //Toggled when the player is under the effect of The Crit-a-Cola. 
    TFCond_InHealRadius = (1 << 20), //Unused condition, name taken from AlliedModders SDK. 
    TFCond_Healing = (1 << 21), //Toggled when someone is being healed by a medic or a dispenser. 
    TFCond_OnFire = (1 << 22), //Toggled when a player is on fire. 
    TFCond_Overhealed = (1 << 23), //Toggled when a player has >100% health. 
    TFCond_Jarated = (1 << 24), //Toggled when a player is hit with a Sniper's Jarate. 
    TFCond_Bleeding = (1 << 25), //Toggled when a player is taking bleeding damage. 
    TFCond_DefenseBuffed = (1 << 26), //Toggled when a player is within range of an activated Battalion's Backup. 
    TFCond_Milked = (1 << 27), //Player was hit with a jar of Mad Milk. 
    TFCond_MegaHeal = (1 << 28), //Player is under the effect of Quick-Fix charge. 
    TFCond_RegenBuffed = (1 << 29), //Toggled when a player is within a Concheror's range. 
    TFCond_MarkedForDeath = (1 << 30), //Player is marked for death by a Fan O'War hit. Effects are similar to TFCond_Jarated. 
	TFCond_NoHealingDamageBuff = (1 << 31), //Unknown what this is used for.

    TFCondEx_SpeedBuffAlly = (1 << 0), //Toggled when a player gets hit with the disciplinary action. 
    TFCondEx_HalloweenCritCandy = (1 << 1), //Only for Scream Fortress event maps that drop crit candy. 
	TFCondEx_CritCanteen = (1 << 2), //Player is getting a crit boost from a MVM canteen.
	TFCondEx_CritDemoCharge = (1 << 3), //From demo's shield
	TFCondEx_CritHype = (1 << 4), //Soda Popper crits. 
    TFCondEx_CritOnFirstBlood = (1 << 5), //Arena first blood crit buff. 
    TFCondEx_CritOnWin = (1 << 6), //End of round crits. 
    TFCondEx_CritOnFlagCapture = (1 << 7), //CTF intelligence capture crits. 
    TFCondEx_CritOnKill = (1 << 8), //Unknown what this is for. 
    TFCondEx_RestrictToMelee = (1 << 9), //Unknown what this is for. 
	TFCondEx_DefenseBuffNoCritBlock = ( 1 << 10 ), //MvM Buff.
	TFCondEx_Reprogrammed = (1 << 11), //MvM Bot has been reprogrammed.
    TFCondEx_PyroCrits = (1 << 12), //Player is getting crits from the Mmmph charge. 
    TFCondEx_PyroHeal = (1 << 13), //Player is being healed from the Mmmph charge. 
	TFCondEx_FocusBuff = (1 << 14), //Player is getting a focus buff.
	TFCondEx_DisguisedRemoved = (1 << 15), //Disguised remove from a bot.
	TFCondEx_MarkedForDeathSilent = (1 << 16), //Player is under the effects of the Escape Plan/Equalizer or GRU.
	TFCondEx_DisguisedAsDispenser = (1 << 17), //Bot is disguised as dispenser.
	TFCondEx_Sapped = (1 << 18), //MvM bot is being sapped.
	TFCondEx_UberchargedHidden = (1 << 19), //MvM Related
	TFCondEx_UberchargedCanteen = (1 << 20), //Player is receiving ?berCharge from a canteen.
	TFCondEx_HalloweenBombHead = (1 << 21), //Player has a bomb on their head from Merasmus.
	TFCondEx_HalloweenThriller = (1 << 22), //Players are forced to dance from Merasmus.
	TFCondEx_BulletCharge = (1 << 26), //Player is receiving 75% reduced damage from bullets.
	TFCondEx_ExplosiveCharge = (1 << 27), //Player is receiving 75% reduced damage from explosives.
	TFCondEx_FireCharge = (1 << 28), //Player is receiving 75% reduced damage from fire.
	TFCondEx_BulletResistance = (1 << 29), //Player is receiving 10% reduced damage from bullets.
	TFCondEx_ExplosiveResistance = (1 << 30), //Player is receiving 10% reduced damage from explosives.
	TFCondEx_FireResistance = (1 << 31), //Player is receiving 10% reduced damage from fire.

	TFCondEx2_Stealthed = (1 << 0),
	TFCondEx2_MedigunDebuff = (1 << 1),
	TFCondEx2_StealthedUserBuffFade = (1 << 2),
	TFCondEx2_BulletImmune = (1 << 3),
	TFCondEx2_BlastImmune = (1 << 4),
	TFCondEx2_FireImmune = (1 << 5),
	TFCondEx2_PreventDeath = (1 << 6),
	TFCondEx2_MVMBotRadiowave = (1 << 7),
	TFCondEx2_HalloweenSpeedBoost = (1 << 8), //Wheel has granted player speed boost.
	TFCondEx2_HalloweenQuickHeal = (1 << 9), //Wheel has granted player quick heal.
	TFCondEx2_HalloweenGiant = (1 << 10), //Wheel has granted player giant mode.
	TFCondEx2_HalloweenTiny = (1 << 11), //Wheel has granted player tiny mode.
	TFCondEx2_HalloweenInHell = (1 << 12), //Wheel has granted player in hell mode.
	TFCondEx2_HalloweenGhostMode = (1 << 13), //Wheel has granted player ghost mode.
	TFCondEx2_Parachute = (1 << 16), //Player has deployed the BASE Jumper.
	TFCondEx2_BlastJumping = (1 << 17), //Player has sticky or rocket jumped.

    TFCond_MiniCrits = ( TFCond_Buffed | TFCond_CritCola ),
    TFCond_IgnoreStates = ( TFCond_Ubercharged | TFCond_Bonked ), 
    TFCondEx_IgnoreStates = ( TFCondEx_PyroHeal ) 
};

enum tf_classes
{
	TF2_Scout = 1,
	TF2_Soldier = 3,
	TF2_Pyro = 7,
	TF2_Demoman = 4,
	TF2_Heavy = 6,
	TF2_Engineer = 9,
	TF2_Medic = 5,
	TF2_Sniper = 2,
	TF2_Spy = 8,
};

enum source_lifestates
{
	LIFE_ALIVE,
	LIFE_DYING,
	LIFE_DEAD,
	LIFE_RESPAWNABLE,
	LIFE_DISCARDBODY,
};

class CInput
{
public:
	CUserCmd* GetUserCmd( int seq )
	{
		typedef CUserCmd* ( __thiscall* OriginalFn )( PVOID, int );
		return getvfunc<OriginalFn>( this, 8 )( this, seq );
	}
};
